pub(crate) mod defaults;
pub mod plugin;

pub trait Linter {
    #[expect(async_fn_in_trait)]
    async fn lint(&self) -> anyhow::Result<()>;
}
