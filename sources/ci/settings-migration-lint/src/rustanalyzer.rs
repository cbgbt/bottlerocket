//! Contains an async rust-analyzer LSP/JSON RPC client.
//!
//! rust-analyzer responds to JSON RPC messages on stdin with responses on stdout.
//! This module supports concurrent requests, even if responses are received out of order.
use jsonrpsee_types::{
    Notification as RPCNotification, Response as RPCResponse, ResponsePayload as RPCResponsePayload,
};
use lsp_types as lsp;
use serde::{Deserialize, Serialize};
use snafu::{OptionExt, ResultExt, Snafu};
use std::collections::HashMap;
use std::num::ParseIntError;
use std::ops::Deref;
use std::path::Path;
use std::process::Stdio;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::io::{AsyncBufRead, AsyncBufReadExt, BufReader};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::{Child, ChildStdout, Command};
use tokio::sync::{Mutex, broadcast, oneshot};

const NOTIFICATION_PUBSUB_CAP: usize = 16;

type ResponseCallbacks = Arc<
    Mutex<
        HashMap<
            jsonrpsee_types::Id<'static>,
            oneshot::Sender<RPCResponsePayload<'static, serde_json::Value>>,
        >,
    >,
>;
pub type NotificationCallback = (String, serde_json::Value);

pub struct RustAnalyzer {
    /// Responses occur asynchronously, so we need to store a mapping of request IDs to callbacks
    /// that will receive the response.
    response_callbacks: ResponseCallbacks,
    notification_callbacks: broadcast::Sender<NotificationCallback>,
    ra_proc: Mutex<Child>,
    request_id: AtomicU64,
}

impl RustAnalyzer {
    /// Start a rust-analyzer subprocess
    pub async fn start(cwd: impl AsRef<Path>) -> Result<Self, RustAnalyzerError> {
        use rust_analyzer_error::*;
        let mut ra_proc = Command::new("rust-analyzer")
            .current_dir(cwd)
            .kill_on_drop(true)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .context(StartProcessSnafu)?;

        let response_callbacks = Arc::new(Mutex::new(HashMap::new()));
        let (notification_callbacks, _) = broadcast::channel(NOTIFICATION_PUBSUB_CAP);

        // TODO detect failures here
        tokio::spawn(Self::service_task(
            ra_proc.stdout.take().unwrap(),
            Arc::clone(&response_callbacks),
            notification_callbacks.clone(),
        ));

        let ra_proc = Mutex::new(ra_proc);
        let request_id = AtomicU64::new(0);
        Ok(Self {
            response_callbacks,
            notification_callbacks,
            ra_proc,
            request_id,
        })
    }

    pub fn notifications(&self) -> broadcast::Receiver<NotificationCallback> {
        self.notification_callbacks.subscribe()
    }

    /// Handle JSON RPC responses on a separate thread.
    ///
    /// When requests are sent, we register callbacks that receive responses for each JSON RPC
    /// request.
    /// Requests are tagged with a unique int, and the responses are tagged with the int for the
    /// call which spawned the response.
    async fn service_task(
        mut msg_stream: ChildStdout,
        response_callbacks: ResponseCallbacks,
        notification_callbacks: broadcast::Sender<NotificationCallback>,
    ) -> Result<(), RustAnalyzerError> {
        use rust_analyzer_error::*;

        let mut reader = BufReader::new(&mut msg_stream);
        loop {
            let headers = Headers::from_reader(&mut reader)
                .await
                .context(ParseHeadersSnafu)?;

            let content_length: usize = headers
                .headers
                .get("Content-Length")
                .context(MissingContentLengthSnafu)?
                .parse()
                .context(ParseContentLengthSnafu)?;

            let mut content = vec![0; content_length];
            reader
                .read_exact(&mut content)
                .await
                .context(ReadResponseSnafu)?;

            let response: IncomingJsonRpcMessage =
                serde_json::from_slice(&content).context(ParseJsonRpcResponseSnafu)?;

            let response = match response {
                // We currently just drop notifications
                IncomingJsonRpcMessage::Notification(notif) => {
                    let callback = (notif.method.to_string(), notif.params);
                    notification_callbacks.send(callback).ok();
                    continue;
                }
                IncomingJsonRpcMessage::Response(response) => response.into_owned(),
            };

            let callback = {
                let mut callbacks = response_callbacks.lock().await;
                callbacks.remove(&response.id)
            };

            // If there's no registered callback or the receiver has dropped, then silently drop the
            // message.
            if let Some(callback) = callback {
                callback.send(response.payload).ok();
            }
        }
    }

    /// Send an LSP request
    pub async fn send_request<Req>(
        &self,
        params: Req::Params,
    ) -> Result<Req::Result, RustAnalyzerError>
    where
        Req: lsp::request::Request,
    {
        use rust_analyzer_error::*;

        let id = jsonrpsee_types::Id::Number(self.request_id.fetch_add(1, Ordering::SeqCst));
        let jsonrpc = jsonrpsee_types::TwoPointZero;

        let params = serde_json::to_value(params).context(SerializeRequestSnafu)?;

        let request = jsonrpsee_types::Request {
            id: id.clone(),
            jsonrpc,
            method: std::borrow::Cow::Borrowed(Req::METHOD),
            params: Some(serde_json::from_value(params).context(SerializeRequestSnafu)?),
            extensions: jsonrpsee_types::Extensions::new(),
        };
        let request_str = serde_json::to_string(&request).context(SerializeRequestSnafu)?;
        let request = format!("Content-Length: {}\r\n\r\n{request_str}", request_str.len());

        let (sender, receiver) = oneshot::channel();
        {
            self.response_callbacks.lock().await.insert(id, sender);
        }
        {
            self.ra_proc
                .lock()
                .await
                .stdin
                .as_mut()
                .context(StdinSnafu)?
                .write_all(request.as_bytes())
                .await
                .context(WriteRequestSnafu)?;
        }

        let rpc_response = receiver
            .await
            .expect("rust-analyzer LSP client has panicked");

        match rpc_response {
            RPCResponsePayload::Success(success) => {
                Ok(serde_json::from_value(success.deref().clone()).context(ParseResponseSnafu)?)
            }
            RPCResponsePayload::Error(error) => Err(RustAnalyzerError::ErrorResponse {
                source: error.into_owned(),
            }),
        }
    }

    /// Send an LSP notification
    pub async fn send_notification<Notif>(
        &self,
        params: Notif::Params,
    ) -> Result<(), RustAnalyzerError>
    where
        Notif: lsp::notification::Notification,
    {
        use rust_analyzer_error::*;

        let jsonrpc = jsonrpsee_types::TwoPointZero;

        let notif = jsonrpsee_types::Notification {
            jsonrpc,
            method: std::borrow::Cow::Borrowed(Notif::METHOD),
            params,
            extensions: jsonrpsee_types::Extensions::new(),
        };
        let request_str = serde_json::to_string(&notif).context(SerializeRequestSnafu)?;
        let request = format!("Content-Length: {}\r\n\r\n{request_str}", request_str.len());

        self.ra_proc
            .lock()
            .await
            .stdin
            .as_mut()
            .context(StdinSnafu)?
            .write_all(request.as_bytes())
            .await
            .context(WriteRequestSnafu)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum IncomingJsonRpcMessage<'a> {
    #[serde(borrow)]
    Response(RPCResponse<'a, serde_json::Value>),
    #[serde(borrow)]
    Notification(RPCNotification<'a, serde_json::Value>),
}

#[derive(Debug, Snafu)]
#[snafu(module)]
pub enum RustAnalyzerError {
    #[snafu(display("Response from rust-analyzer missing 'Content-Length' response header"))]
    MissingContentLength,

    #[snafu(display(
        "Unable to parse content length from rust-analyzer response header: {source}"
    ))]
    ParseContentLength { source: ParseIntError },

    #[snafu(display("Unable to read headers from rust-analyzer response: {source}"))]
    ParseHeaders { source: ReadHeadersError },

    #[snafu(display("Unable to parse JSON-RPC response from rust-analyzer: {source}"))]
    ParseJsonRpcResponse { source: serde_json::Error },

    #[snafu(display("Unable to parse LSP response from rust-analyzer: {source}"))]
    ParseResponse { source: serde_json::Error },

    #[snafu(display("Unable to read response from rust-analyzer: {source}"))]
    ReadResponse { source: std::io::Error },

    #[snafu(display("Request to rust-analyzer returned an error: {source}"))]
    ErrorResponse {
        source: jsonrpsee_types::ErrorObjectOwned,
    },

    #[snafu(display("Unable to serialize request to rust-analyzer: {source}"))]
    SerializeRequest { source: serde_json::Error },

    #[snafu(display("Unable to start rust-analyzer: {source}"))]
    StartProcess { source: std::io::Error },

    #[snafu(display("Unable to send request to rust-analyzer: missing stdin for process"))]
    Stdin,

    #[snafu(display("Unable to write request to rust-analyzer: {source}"))]
    WriteRequest { source: std::io::Error },
}

/// rust-analyzer's JSON RPC responses are prepended by HTTP-style headers.
///
/// Currently, the only header that is used is `Content-Length`, which is the length of the
/// response body; however, we've future-proofed the implementation against future header additions.
struct Headers {
    pub headers: HashMap<String, String>,
}

impl Headers {
    pub async fn from_reader<R: AsyncBufRead + Unpin>(
        reader: &mut R,
    ) -> Result<Self, ReadHeadersError> {
        use read_headers_error::*;

        let mut headers = HashMap::new();
        loop {
            let mut header = String::new();
            reader.read_line(&mut header).await.context(ReadSnafu)?;
            if header.trim().is_empty() {
                break;
            }
            let header = header.parse::<Header>().context(ParseSnafu)?;
            headers.insert(header.key, header.value);
        }
        Ok(Self { headers })
    }
}

#[derive(Debug, Snafu)]
#[snafu(module)]
pub enum ReadHeadersError {
    #[snafu(display("Unable to read headers: {source}"))]
    Read { source: std::io::Error },

    #[snafu(display("Unable to parse headers: {source}"))]
    Parse { source: ParseHeaderError },
}

struct Header {
    pub key: String,
    pub value: String,
}

impl FromStr for Header {
    type Err = ParseHeaderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use parse_header_error::*;

        let (key, value) = s.split_once(": ").context(NoDelimiterSnafu)?;
        Ok(Self {
            key: key.trim().to_string(),
            value: value.trim().to_string(),
        })
    }
}

#[derive(Debug, Snafu)]
#[snafu(module)]
pub enum ParseHeaderError {
    #[snafu(display("Missing colon delimiter"))]
    NoDelimiter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerStatus {
    pub health: String,
    pub message: Option<String>,
    pub quiescent: bool,
}
