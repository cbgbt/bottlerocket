/*!
# API models

Bottlerocket has different variants supporting different features and use cases.
Each variant has its own set of software, and therefore needs its own configuration.
We support having an API model for each variant to support these different configurations.

Each model defines a top-level `Settings` structure.
It can use pre-defined structures inside, or custom ones as needed.

This `Settings` essentially becomes the schema for the variant's data store.
`apiserver::datastore` offers serialization and deserialization modules that make it easy to map between Rust types and the data store, and thus, all inputs and outputs are type-checked.

At the field level, standard Rust types can be used, or ["modeled types"](src/modeled_types) that add input validation.

Default values are specified in .toml files in each variant's `defaults.d` directory under [src](src).
(For example, see the [aws-ecs-1 defaults](src/aws-ecs-1/defaults.d/).)
Entries are sorted by filename, and later entries take precedence.

The `#[model]` attribute on Settings and its sub-structs reduces duplication and adds some required metadata; see [its docs](model-derive/) for details.

## aws-k8s-1.23: Kubernetes 1.23

* [Model](src/aws-k8s-1.25/mod.rs)
* [Default settings](src/aws-k8s-1.25/defaults.d/)

## aws-k8s-1.23-nvidia: Kubernetes 1.23 NVIDIA

* [Model](src/aws-k8s-1.25-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.25-nvidia/defaults.d/)

## aws-k8s-1.24: Kubernetes 1.24

* [Model](src/aws-k8s-1.25/mod.rs)
* [Default settings](src/aws-k8s-1.25/defaults.d/)

## aws-k8s-1.24-nvidia: Kubernetes 1.24 NVIDIA

* [Model](src/aws-k8s-1.25-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.25-nvidia/defaults.d/)

## aws-k8s-1.25: Kubernetes 1.25

* [Model](src/aws-k8s-1.25/mod.rs)
* [Default settings](src/aws-k8s-1.25/defaults.d/)

## aws-k8s-1.25-nvidia: Kubernetes 1.25 NVIDIA

* [Model](src/aws-k8s-1.25-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.25-nvidia/defaults.d/)

## aws-k8s-1.26: Kubernetes 1.26

* [Model](src/aws-k8s-1.26/mod.rs)
* [Default settings](src/aws-k8s-1.26/defaults.d/)

## aws-k8s-1.26-nvidia: Kubernetes 1.26 NVIDIA

* [Model](src/aws-k8s-1.26-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.26-nvidia/defaults.d/)

## aws-k8s-1.27: Kubernetes 1.27

* [Model](src/aws-k8s-1.27/mod.rs)
* [Default settings](src/aws-k8s-1.27/defaults.d/)

## aws-k8s-1.27-nvidia: Kubernetes 1.27 NVIDIA

* [Model](src/aws-k8s-1.27-nvidia/mod.rs)
* [Default settings](src/aws-k8s-1.27-nvidia/defaults.d/)

## aws-ecs-1: Amazon ECS

* [Model](src/aws-ecs-1/mod.rs)
* [Default settings](src/aws-ecs-1/defaults.d/)

## aws-ecs-1-nvidia: Amazon ECS NVIDIA

* [Model](src/aws-ecs-1-nvidia/mod.rs)
* [Default settings](src/aws-ecs-1-nvidia/defaults.d/)

## aws-ecs-2: Amazon ECS

* [Model](src/aws-ecs-1/mod.rs)
* [Default settings](src/aws-ecs-1/defaults.d/)

## aws-ecs-2-nvidia: Amazon ECS NVIDIA

* [Model](src/aws-ecs-1-nvidia/mod.rs)
* [Default settings](src/aws-ecs-1-nvidia/defaults.d/)

## aws-dev: AWS development build

* [Model](src/aws-dev/mod.rs)
* [Default settings](src/aws-dev/defaults.d/)

## vmware-dev: VMware development build

* [Model](src/vmware-dev/mod.rs)
* [Default settings](src/vmware-dev/defaults.d/)

## vmware-k8s-1.23: VMware Kubernetes 1.23

* [Model](src/vmware-k8s-1.27/mod.rs)
* [Default settings](src/vmware-k8s-1.27/defaults.d/)

## vmware-k8s-1.24: VMware Kubernetes 1.24

* [Model](src/vmware-k8s-1.27/mod.rs)
* [Default settings](src/vmware-k8s-1.27/defaults.d/)

## vmware-k8s-1.25: VMware Kubernetes 1.25

* [Model](src/vmware-k8s-1.27/mod.rs)
* [Default settings](src/vmware-k8s-1.27/defaults.d/)

## vmware-k8s-1.26: VMware Kubernetes 1.26

* [Model](src/vmware-k8s-1.27/mod.rs)
* [Default settings](src/vmware-k8s-1.27/defaults.d/)

## vmware-k8s-1.27: VMware Kubernetes 1.27

* [Model](src/vmware-k8s-1.27/mod.rs)
* [Default settings](src/vmware-k8s-1.27/defaults.d/)

## metal-dev: Metal development build

* [Model](src/metal-dev/mod.rs)
* [Default settings](src/metal-dev/defaults.d/)

## metal-k8s-1.23: Metal Kubernetes 1.23

* [Model](src/metal-k8s-1.27/mod.rs)
* [Default settings](src/metal-k8s-1.27/defaults.d/)

## metal-k8s-1.24: Metal Kubernetes 1.24

* [Model](src/metal-k8s-1.27/mod.rs)
* [Default settings](src/metal-k8s-1.27/defaults.d/)

## metal-k8s-1.25: Metal Kubernetes 1.25

* [Model](src/metal-k8s-1.27/mod.rs)
* [Default settings](src/metal-k8s-1.27/defaults.d/)

## metal-k8s-1.26: Metal Kubernetes 1.26

* [Model](src/metal-k8s-1.27/mod.rs)
* [Default settings](src/metal-k8s-1.27/defaults.d/)

## metal-k8s-1.27: Metal Kubernetes 1.27

* [Model](src/metal-k8s-1.27/mod.rs)
* [Default settings](src/metal-k8s-1.27/defaults.d/)

# This directory

We use `build.rs` to symlink the proper API model source code for Cargo to build.
We determine the "proper" model by using the `VARIANT` environment variable.

If a developer is doing a local `cargo build`, they need to set `VARIANT`.

When building with the Bottlerocket build system, `VARIANT` is based on `BUILDSYS_VARIANT` from the top-level `Makefile.toml`, which can be overridden on the command line with `cargo make -e BUILDSYS_VARIANT=bla`.

Note: when building with the build system, we can't create the symlink in the source directory during a build - the directories are owned by `root`, but we're `builder`.
We can't use a read/write bind mount with current Docker syntax.
To get around this, in the top-level `Dockerfile`, we mount a "cache" directory at `src/variant` that we can modify, and create a `current` symlink inside.
The code in `src/lib.rs` then imports the requested model using `variant/current`.

Note: for the same reason, we symlink `variant/mod.rs` to `variant_mod.rs`.
Rust needs a `mod.rs` file to understand that a directory is part of the module structure, so we have to have `variant/mod.rs`.
`variant/` is the cache mount that starts empty, so we have to store the file elsewhere and link it in.

Note: all models share the same `Cargo.toml`.
*/

// Clippy has a false positive in the presence of the Scalar macro.
#![allow(clippy::derived_hash_with_manual_eq)]
#![macro_use]

pub mod aws_dev;
pub mod aws_ecs_1;
pub mod aws_ecs_1_nvidia;
pub mod aws_ecs_2;
pub mod aws_ecs_2_nvidia;
pub mod aws_k8s_1_23;
pub mod aws_k8s_1_23_nvidia;
pub mod aws_k8s_1_24;
pub mod aws_k8s_1_24_nvidia;
pub mod aws_k8s_1_25;
pub mod aws_k8s_1_25_nvidia;
pub mod aws_k8s_1_26;
pub mod aws_k8s_1_26_nvidia;
pub mod aws_k8s_1_27;
pub mod aws_k8s_1_27_nvidia;
pub mod metal_dev;
pub mod metal_k8s_1_23;
pub mod metal_k8s_1_24;
pub mod metal_k8s_1_25;
pub mod metal_k8s_1_26;
pub mod metal_k8s_1_27;
pub mod vmware_dev;
pub mod vmware_k8s_1_23;
pub mod vmware_k8s_1_24;
pub mod vmware_k8s_1_25;
pub mod vmware_k8s_1_26;
pub mod vmware_k8s_1_27;

macro_rules! randogen_hashmap {
    ($k:tt, $v:tt) => {
        impl RandoHashmap<$k, $v> {
            pub fn generate<R: rand::Rng + ?Sized>(
                rng: &mut R,
                min: usize,
                max: usize,
            ) -> HashMap<$k, $v> {
                (0..rng.gen_range(min..max))
                    .map(|_| ($k::generate_random(), $v::generate_random()))
                    .collect()
            }
        }
    };
}

macro_rules! randogen_vec {
    ($t:tt) => {
        impl RandoVec<$t> {
            pub fn generate<R: rand::Rng + ?Sized>(rng: &mut R, min: usize, max: usize) -> Vec<$t> {
                (0..rng.gen_range(min..max))
                    .map(|_| $t::generate_random())
                    .collect()
            }
        }
    };
}

// "Modeled types" are types with special ser/de behavior used for validation.
pub mod modeled_types;

// The "variant" module is just a directory where we symlink in the user's requested build
// variant; each variant defines a top-level Settings structure and we re-export the current one.
mod variant;
// The "de" module contains custom deserialization trait implementation for models.
mod de;

use modeled_types::KubernetesCPUManagerPolicyOption;
use modeled_types::KubernetesEvictionKey;
use modeled_types::KubernetesMemoryManagerPolicy;
use modeled_types::KubernetesMemoryReservation;
use modeled_types::NonNegativeInteger;
pub use variant::*;

// Types used to communicate between client and server for 'apiclient exec'.
pub mod exec;

// Below, we define common structures used in the API surface; specific variants build a Settings
// structure based on these, and that's what gets exposed via the API.  (Specific variants' models
// are in subdirectories and linked into place by build.rs at variant/current.)

use model_derive::model;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::Ipv4Addr, sync::Mutex};
use std::{
    marker::PhantomData,
    net::{IpAddr, Ipv6Addr},
};

use crate::de::{deserialize_limit, deserialize_mirrors, deserialize_node_taints};
use crate::modeled_types::{
    BootConfigKey, BootConfigValue, BootstrapContainerMode, CpuManagerPolicy, CredentialProvider,
    DNSDomain, ECSAgentImagePullBehavior, ECSAgentLogLevel, ECSAttributeKey, ECSAttributeValue,
    ECSDurationValue, EtcHostsEntries, FriendlyVersion, Identifier, IntegerPercent, KmodKey,
    KubernetesAuthenticationMode, KubernetesBootstrapToken, KubernetesCloudProvider,
    KubernetesClusterDnsIp, KubernetesClusterName, KubernetesDurationValue, KubernetesLabelKey,
    KubernetesLabelValue, KubernetesQuantityValue, KubernetesReservedResourceKey,
    KubernetesTaintValue, KubernetesThresholdValue, Lockdown, OciDefaultsCapability,
    OciDefaultsResourceLimitType, PemCertificateString, SingleLineString, SysctlKey,
    TopologyManagerPolicy, TopologyManagerScope, Url, ValidBase64, ValidLinuxHostname,
};

struct RandoHashmap<K, V> {
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}

struct RandoVec<T> {
    _t: PhantomData<T>,
}

pub fn rando_alphanumeric<R: rand::Rng + ?Sized>(rng: &mut R) -> String {
    (5u8..rng.gen())
        .map(|_| rng.sample(&rand::distributions::Alphanumeric))
        .map(char::from)
        .collect()
}

pub fn rando_alphanumeric_constrained<R: rand::Rng + ?Sized>(
    rng: &mut R,
    min: u64,
    max: u64,
) -> String {
    (0u64..rng.gen_range(min..max))
        .map(|_| rng.sample(&rand::distributions::Alphanumeric))
        .map(char::from)
        .collect()
}

pub fn rando_domain<R: rand::Rng + ?Sized>(rng: &mut R) -> String {
    format!(
        "{}.{}",
        crate::rando_alphanumeric_constrained(rng, 2, 10),
        crate::rando_alphanumeric_constrained(rng, 2, 3)
    )
    .to_ascii_lowercase()
}

pub fn rando_ipaddr<R: rand::Rng + ?Sized>(rng: &mut R) -> IpAddr {
    if rng.gen::<bool>() {
        IpAddr::V4(Ipv4Addr::new(rng.gen(), rng.gen(), rng.gen(), rng.gen()))
    } else {
        IpAddr::V6(Ipv6Addr::new(
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
        ))
    }
}

pub fn rando_ipaddrs<R: rand::Rng + ?Sized>(rng: &mut R, min: usize, max: usize) -> Vec<IpAddr> {
    (0..rng.gen_range(min..max))
        .map(|_| rando_ipaddr(rng))
        .collect()
}

pub fn rando_duration<R: rand::Rng + ?Sized>(rng: &mut R) -> String {
    [
        format!("{}ms", rng.gen_range(0..1000)),
        format!("{}s", rng.gen_range(0..60)),
        format!("{}m", rng.gen_range(0..60)),
        format!("{}h", rng.gen_range(0..24)),
        format!(
            "{}h{}m{}s{}ms",
            rng.gen_range(0..24),
            rng.gen_range(0..60),
            rng.gen_range(0..60),
            rng.gen_range(0..1000)
        ),
        format!("{}s{}ms", rng.gen_range(0..60), rng.gen_range(0..1000)),
    ][rng.gen_range(0..6)]
    .to_string()
}

// Kubernetes static pod manifest settings
#[model]
struct StaticPod {
    enabled: bool,
    manifest: ValidBase64,
}

// Kubernetes related settings. The dynamic settings are retrieved from
// IMDS via Sundog's child "Pluto".
#[model]
struct KubernetesSettings {
    // Settings that must be specified via user data or through API requests.  Not all settings are
    // useful for all modes. For example, in standalone mode the user does not need to specify any
    // cluster information, and the bootstrap token is only needed for TLS authentication mode.
    cluster_name: KubernetesClusterName,
    cluster_certificate: ValidBase64,
    api_server: Url,
    #[rand_derive(custom)]
    node_labels: HashMap<KubernetesLabelKey, KubernetesLabelValue>,
    #[serde(default, deserialize_with = "deserialize_node_taints")]
    #[rand_derive(custom)]
    node_taints: Option<HashMap<KubernetesLabelKey, Vec<KubernetesTaintValue>>>,
    #[rand_derive(custom)]
    static_pods: HashMap<Identifier, StaticPod>,
    authentication_mode: KubernetesAuthenticationMode,
    bootstrap_token: KubernetesBootstrapToken,
    standalone_mode: bool,
    #[rand_derive(custom)]
    eviction_hard: HashMap<KubernetesEvictionKey, KubernetesThresholdValue>,
    #[rand_derive(custom)]
    eviction_soft: HashMap<KubernetesEvictionKey, KubernetesThresholdValue>,
    #[rand_derive(custom)]
    eviction_soft_grace_period: HashMap<KubernetesEvictionKey, KubernetesDurationValue>,
    eviction_max_pod_grace_period: NonNegativeInteger,
    #[rand_derive(custom)]
    kube_reserved: HashMap<KubernetesReservedResourceKey, KubernetesQuantityValue>,
    #[rand_derive(custom)]
    system_reserved: HashMap<KubernetesReservedResourceKey, KubernetesQuantityValue>,
    #[rand_derive(custom)]
    allowed_unsafe_sysctls: Vec<SingleLineString>,
    server_tls_bootstrap: bool,
    cloud_provider: KubernetesCloudProvider,
    registry_qps: i32,
    registry_burst: i32,
    event_qps: i32,
    event_burst: i32,
    kube_api_qps: i32,
    kube_api_burst: i32,
    container_log_max_size: KubernetesQuantityValue,
    container_log_max_files: i32,
    cpu_cfs_quota_enforced: bool,
    cpu_manager_policy: CpuManagerPolicy,
    cpu_manager_reconcile_period: KubernetesDurationValue,
    #[rand_derive(custom)]
    cpu_manager_policy_options: Vec<KubernetesCPUManagerPolicyOption>,
    topology_manager_scope: TopologyManagerScope,
    topology_manager_policy: TopologyManagerPolicy,
    pod_pids_limit: i64,
    image_gc_high_threshold_percent: IntegerPercent,
    image_gc_low_threshold_percent: IntegerPercent,
    provider_id: Url,
    log_level: u8,
    #[rand_derive(custom)]
    credential_providers: HashMap<Identifier, CredentialProvider>,
    server_certificate: ValidBase64,
    server_key: ValidBase64,
    shutdown_grace_period: KubernetesDurationValue,
    shutdown_grace_period_for_critical_pods: KubernetesDurationValue,
    #[rand_derive(custom)]
    memory_manager_reserved_memory: HashMap<Identifier, KubernetesMemoryReservation>,
    memory_manager_policy: KubernetesMemoryManagerPolicy,

    // Settings where we generate a value based on the runtime environment.  The user can specify a
    // value to override the generated one, but typically would not.
    max_pods: u32,
    cluster_dns_ip: KubernetesClusterDnsIp,
    cluster_domain: DNSDomain,
    #[rand_derive(custom)]
    node_ip: IpAddr,
    pod_infra_container_image: SingleLineString,
    // Generated in `aws-k8s-1.26*` variants only
    hostname_override: ValidLinuxHostname,
    // Generated in `k8s-1.25+` variants only
    seccomp_default: bool,
}

randogen_hashmap!(Identifier, HostContainer);
randogen_hashmap!(Identifier, BootstrapContainer);
randogen_hashmap!(Identifier, PemCertificate);

randogen_hashmap!(KubernetesLabelKey, KubernetesLabelValue);
randogen_hashmap!(Identifier, StaticPod);
randogen_hashmap!(KubernetesEvictionKey, KubernetesThresholdValue);
randogen_hashmap!(KubernetesEvictionKey, KubernetesDurationValue);
randogen_hashmap!(KubernetesReservedResourceKey, KubernetesQuantityValue);
randogen_hashmap!(Identifier, CredentialProvider);
randogen_hashmap!(Identifier, KubernetesMemoryReservation);

randogen_vec!(Url);
randogen_vec!(SingleLineString);
randogen_vec!(KubernetesCPUManagerPolicyOption);
randogen_vec!(ValidLinuxHostname);

fn maybe_return<R: rand::Rng + ?Sized, T>(rng: &mut R, value: T) -> T {
    value
}

impl TestDataProviderForKubernetesSettings for KubernetesSettings {
    fn generate_node_labels<R: rand::Rng + ?Sized>(
        rng: &mut R,
    ) -> HashMap<KubernetesLabelKey, KubernetesLabelValue> {
        let r = RandoHashmap::<KubernetesLabelKey, KubernetesLabelValue>::generate(rng, 2, 5);
        maybe_return(rng, r)
    }

    fn generate_node_taints<R: rand::Rng + ?Sized>(
        rng: &mut R,
    ) -> Option<HashMap<KubernetesLabelKey, Vec<KubernetesTaintValue>>> {
        let r = RandoHashmap::<KubernetesLabelKey, Vec<KubernetesTaintValue>>::generate(rng, 2, 5);
        Some(maybe_return(rng, r))
    }

    fn generate_static_pods<R: rand::Rng + ?Sized>(rng: &mut R) -> HashMap<Identifier, StaticPod> {
        let r = RandoHashmap::<Identifier, StaticPod>::generate(rng, 2, 5);
        maybe_return(rng, r)
    }

    fn generate_eviction_hard<R: rand::Rng + ?Sized>(
        rng: &mut R,
    ) -> HashMap<KubernetesEvictionKey, KubernetesThresholdValue> {
        let r =
            RandoHashmap::<KubernetesEvictionKey, KubernetesThresholdValue>::generate(rng, 2, 5);
        maybe_return(rng, r)
    }

    fn generate_eviction_soft<R: rand::Rng + ?Sized>(
        rng: &mut R,
    ) -> HashMap<KubernetesEvictionKey, KubernetesThresholdValue> {
        let r =
            RandoHashmap::<KubernetesEvictionKey, KubernetesThresholdValue>::generate(rng, 2, 5);
        maybe_return(rng, r)
    }

    fn generate_eviction_soft_grace_period<R: rand::Rng + ?Sized>(
        rng: &mut R,
    ) -> HashMap<KubernetesEvictionKey, KubernetesDurationValue> {
        let r = RandoHashmap::<KubernetesEvictionKey, KubernetesDurationValue>::generate(rng, 2, 5);
        maybe_return(rng, r)
    }

    fn generate_kube_reserved<R: rand::Rng + ?Sized>(
        rng: &mut R,
    ) -> HashMap<KubernetesReservedResourceKey, KubernetesQuantityValue> {
        let r = RandoHashmap::<KubernetesReservedResourceKey, KubernetesQuantityValue>::generate(
            rng, 2, 5,
        );
        maybe_return(rng, r)
    }

    fn generate_system_reserved<R: rand::Rng + ?Sized>(
        rng: &mut R,
    ) -> HashMap<KubernetesReservedResourceKey, KubernetesQuantityValue> {
        let r = RandoHashmap::<KubernetesReservedResourceKey, KubernetesQuantityValue>::generate(
            rng, 2, 5,
        );
        maybe_return(rng, r)
    }

    fn generate_allowed_unsafe_sysctls<R: rand::Rng + ?Sized>(
        rng: &mut R,
    ) -> Vec<SingleLineString> {
        let r = RandoVec::<SingleLineString>::generate(rng, 2, 5);
        maybe_return(rng, r)
    }

    fn generate_cpu_manager_policy_options<R: rand::Rng + ?Sized>(
        rng: &mut R,
    ) -> Vec<KubernetesCPUManagerPolicyOption> {
        let r = RandoVec::<KubernetesCPUManagerPolicyOption>::generate(rng, 2, 5);
        maybe_return(rng, r)
    }

    fn generate_credential_providers<R: rand::Rng + ?Sized>(
        rng: &mut R,
    ) -> HashMap<Identifier, CredentialProvider> {
        let r = RandoHashmap::<Identifier, CredentialProvider>::generate(rng, 2, 5);
        maybe_return(rng, r)
    }

    fn generate_memory_manager_reserved_memory<R: rand::Rng + ?Sized>(
        rng: &mut R,
    ) -> HashMap<Identifier, KubernetesMemoryReservation> {
        let r = RandoHashmap::<Identifier, KubernetesMemoryReservation>::generate(rng, 2, 5);
        maybe_return(rng, r)
    }

    fn generate_node_ip<R: rand::Rng + ?Sized>(rng: &mut R) -> IpAddr {
        let r = rando_ipaddr(rng);
        maybe_return(rng, r)
    }
}

// ECS settings.
#[model]
struct ECSSettings {
    cluster: String,
    #[rand_derive(custom)]
    instance_attributes: HashMap<ECSAttributeKey, ECSAttributeValue>,
    allow_privileged_containers: bool,
    #[rand_derive(custom)]
    logging_drivers: Vec<SingleLineString>,
    loglevel: ECSAgentLogLevel,
    enable_spot_instance_draining: bool,
    image_pull_behavior: ECSAgentImagePullBehavior,
    container_stop_timeout: ECSDurationValue,
    task_cleanup_wait: ECSDurationValue,
    metadata_service_rps: i64,
    metadata_service_burst: i64,
    reserved_memory: u16,
    image_cleanup_wait: ECSDurationValue,
    image_cleanup_delete_per_cycle: i64,
    image_cleanup_enabled: bool,
    image_cleanup_age: ECSDurationValue,
}

randogen_hashmap!(ECSAttributeKey, ECSAttributeValue);

impl TestDataProviderForECSSettings for ECSSettings {
    fn generate_instance_attributes<R: rand::Rng + ?Sized>(
        rng: &mut R,
    ) -> HashMap<ECSAttributeKey, ECSAttributeValue> {
        let r = RandoHashmap::<ECSAttributeKey, ECSAttributeValue>::generate(rng, 2, 5);
        maybe_return(rng, r)
    }

    fn generate_logging_drivers<R: rand::Rng + ?Sized>(rng: &mut R) -> Vec<SingleLineString> {
        let r = RandoVec::<SingleLineString>::generate(rng, 2, 5);
        maybe_return(rng, r)
    }
}

#[model]
struct RegistryMirror {
    registry: SingleLineString,
    #[rand_derive(custom)]
    endpoint: Vec<Url>,
}

impl TestDataProviderForRegistryMirror for RegistryMirror {
    fn generate_endpoint<R: rand::Rng + ?Sized>(rng: &mut R) -> Vec<Url> {
        let r = RandoVec::<Url>::generate(rng, 2, 5);
        maybe_return(rng, r)
    }
}

#[model]
struct RegistryCredential {
    registry: SingleLineString,
    username: SingleLineString,
    password: SingleLineString,
    // This is the base64 encoding of "username:password"
    auth: ValidBase64,
    identitytoken: SingleLineString,
}

// Image registry settings for the container runtimes.
#[model]
struct RegistrySettings {
    #[serde(default, deserialize_with = "deserialize_mirrors")]
    #[rand_derive(custom)]
    mirrors: Option<Vec<RegistryMirror>>,
    #[serde(alias = "creds", default)]
    #[rand_derive(custom)]
    credentials: Vec<RegistryCredential>,
}

randogen_vec!(RegistryCredential);
randogen_vec!(RegistryMirror);

impl RandoVec<IpAddr> {
    pub fn generate<R: rand::Rng + ?Sized>(rng: &mut R, min: usize, max: usize) -> Vec<IpAddr> {
        (0..rng.gen_range(min..max))
            .map(|_| rando_ipaddr(rng))
            .collect()
    }
}

impl TestDataProviderForRegistrySettings for RegistrySettings {
    fn generate_credentials<R: rand::Rng + ?Sized>(rng: &mut R) -> Vec<RegistryCredential> {
        let r = RandoVec::<RegistryCredential>::generate(rng, 2, 5);
        maybe_return(rng, r)
    }

    fn generate_mirrors<R: rand::Rng + ?Sized>(rng: &mut R) -> Option<Vec<RegistryMirror>> {
        let r = RandoVec::<RegistryMirror>::generate(rng, 2, 5);
        Some(maybe_return(rng, r))
    }
}

// Update settings. Taken from userdata. The 'seed' setting is generated
// by the "Bork" settings generator at runtime.
#[model(impl_default = true)]
struct UpdatesSettings {
    metadata_base_url: Url,
    targets_base_url: Url,
    seed: u32,
    // Version to update to when updating via the API.
    version_lock: FriendlyVersion,
    ignore_waves: bool,
}

#[model]
struct HostContainer {
    source: Url,
    enabled: bool,
    superpowered: bool,
    user_data: ValidBase64,
}

// Network settings. These settings will affect host service components' network behavior
#[model]
struct NetworkSettings {
    hostname: ValidLinuxHostname,
    hosts: EtcHostsEntries,
    https_proxy: Url,
    // We allow some flexibility in NO_PROXY values because different services support different formats.
    #[rand_derive(custom)]
    no_proxy: Vec<SingleLineString>,
}

impl TestDataProviderForNetworkSettings for NetworkSettings {
    fn generate_no_proxy<R: rand::Rng + ?Sized>(rng: &mut R) -> Vec<SingleLineString> {
        let r = RandoVec::<SingleLineString>::generate(rng, 2, 5);
        maybe_return(rng, r)
    }
}

// NTP settings
#[model]
struct NtpSettings {
    #[rand_derive(custom)]
    time_servers: Vec<Url>,
}

impl TestDataProviderForNtpSettings for NtpSettings {
    fn generate_time_servers<R: rand::Rng + ?Sized>(rng: &mut R) -> Vec<Url> {
        let r = RandoVec::<Url>::generate(rng, 2, 5);
        maybe_return(rng, r)
    }
}

// DNS Settings
#[model]
struct DnsSettings {
    #[rand_derive(custom)]
    name_servers: Vec<IpAddr>,
    #[rand_derive(custom)]
    search_list: Vec<ValidLinuxHostname>,
}

impl TestDataProviderForDnsSettings for DnsSettings {
    fn generate_name_servers<R: rand::Rng + ?Sized>(rng: &mut R) -> Vec<IpAddr> {
        let r = RandoVec::<IpAddr>::generate(rng, 2, 5);
        maybe_return(rng, r)
    }

    fn generate_search_list<R: rand::Rng + ?Sized>(rng: &mut R) -> Vec<ValidLinuxHostname> {
        let r = RandoVec::<ValidLinuxHostname>::generate(rng, 2, 5);
        maybe_return(rng, r)
    }
}

// Kernel settings
#[model]
struct KernelSettings {
    lockdown: Lockdown,
    #[rand_derive(custom)]
    modules: HashMap<KmodKey, KmodSetting>,
    // Values are almost always a single line and often just an integer... but not always.
    #[rand_derive(custom)]
    sysctl: HashMap<SysctlKey, String>,
}

randogen_hashmap!(KmodKey, KmodSetting);

impl RandoHashmap<SysctlKey, String> {
    pub fn generate<R: rand::Rng + ?Sized>(
        rng: &mut R,
        min: usize,
        max: usize,
    ) -> HashMap<SysctlKey, String> {
        (0..rng.gen_range(min..max))
            .map(|_| {
                (
                    SysctlKey::generate_random(),
                    rando_alphanumeric_constrained(rng, 2, 5),
                )
            })
            .collect()
    }
}

impl RandoHashmap<SysctlKey, Vec<String>> {
    pub fn generate<R: rand::Rng + ?Sized>(
        rng: &mut R,
        min: usize,
        max: usize,
    ) -> HashMap<SysctlKey, Vec<String>> {
        (0..rng.gen_range(min..max))
            .map(|_| {
                (
                    SysctlKey::generate_random(),
                    (0..rng.gen_range(2..5))
                        .map(|_| rando_alphanumeric_constrained(rng, 2, 5))
                        .collect(),
                )
            })
            .collect()
    }
}

impl TestDataProviderForKernelSettings for KernelSettings {
    fn generate_sysctl<R: rand::Rng + ?Sized>(rng: &mut R) -> HashMap<SysctlKey, String> {
        let r = RandoHashmap::<SysctlKey, String>::generate(rng, 2, 5);
        maybe_return(rng, r)
    }

    fn generate_modules<R: rand::Rng + ?Sized>(rng: &mut R) -> HashMap<KmodKey, KmodSetting> {
        let r = RandoHashmap::<KmodKey, KmodSetting>::generate(rng, 2, 5);
        maybe_return(rng, r)
    }
}

// Kernel module settings
#[model]
struct KmodSetting {
    allowed: bool,
}

// Kernel boot settings
#[model]
struct BootSettings {
    reboot_to_reconcile: bool,
    #[serde(
        alias = "kernel",
        rename(serialize = "kernel"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[rand_derive(custom)]
    kernel_parameters: Option<HashMap<BootConfigKey, Vec<BootConfigValue>>>,
    #[serde(
        alias = "init",
        rename(serialize = "init"),
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[rand_derive(custom)]
    init_parameters: Option<HashMap<BootConfigKey, Vec<BootConfigValue>>>,
}

randogen_vec!(KubernetesTaintValue);
randogen_vec!(BootConfigValue);

impl RandoHashmap<KubernetesLabelKey, Vec<KubernetesTaintValue>> {
    pub fn generate<R: rand::Rng + ?Sized>(
        rng: &mut R,
        min: usize,
        max: usize,
    ) -> HashMap<KubernetesLabelKey, Vec<KubernetesTaintValue>> {
        (0..rng.gen_range(min..max))
            .map(|_| {
                (
                    KubernetesLabelKey::generate_random(),
                    RandoVec::<KubernetesTaintValue>::generate(rng, 2, 5),
                )
            })
            .collect()
    }
}

impl RandoHashmap<BootConfigKey, Vec<BootConfigValue>> {
    pub fn generate<R: rand::Rng + ?Sized>(
        rng: &mut R,
        min: usize,
        max: usize,
    ) -> HashMap<BootConfigKey, Vec<BootConfigValue>> {
        (0..rng.gen_range(min..max))
            .map(|_| {
                (
                    BootConfigKey::generate_random(),
                    RandoVec::<BootConfigValue>::generate(rng, 2, 5),
                )
            })
            .collect()
    }
}

impl TestDataProviderForBootSettings for BootSettings {
    fn generate_kernel_parameters<R: rand::Rng + ?Sized>(
        rng: &mut R,
    ) -> Option<HashMap<BootConfigKey, Vec<BootConfigValue>>> {
        let r = RandoHashmap::<BootConfigKey, Vec<BootConfigValue>>::generate(rng, 2, 5);
        Some(maybe_return(rng, r))
    }

    fn generate_init_parameters<R: rand::Rng + ?Sized>(
        rng: &mut R,
    ) -> Option<HashMap<BootConfigKey, Vec<BootConfigValue>>> {
        let r = RandoHashmap::<BootConfigKey, Vec<BootConfigValue>>::generate(rng, 2, 5);
        Some(maybe_return(rng, r))
    }
}

// Platform-specific settings
#[model]
struct AwsSettings {
    region: SingleLineString,
    config: ValidBase64,
    credentials: ValidBase64,
    profile: SingleLineString,
}

// Metrics settings
#[model]
struct MetricsSettings {
    metrics_url: Url,
    send_metrics: bool,
    #[rand_derive(custom)]
    service_checks: Vec<String>,
}

impl TestDataProviderForMetricsSettings for MetricsSettings {
    fn generate_service_checks<R: rand::Rng + ?Sized>(rng: &mut R) -> Vec<String> {
        let r = (0..rng.gen_range(2..5))
            .map(|_| crate::rando_alphanumeric_constrained(rng, 5, 30))
            .collect();
        maybe_return(rng, r)
    }
}

// CloudFormation settings
#[model]
struct CloudFormationSettings {
    should_signal: bool,
    stack_name: SingleLineString,
    logical_resource_id: SingleLineString,
}

// AutoScaling settings
#[model]
struct AutoScalingSettings {
    should_wait: bool,
}

// Container runtime settings
#[model]
struct ContainerRuntimeSettings {
    max_container_log_line_size: i32,
    max_concurrent_downloads: i32,
    enable_unprivileged_ports: bool,
    enable_unprivileged_icmp: bool,
}

///// Internal services

// Note: Top-level objects that get returned from the API should have a "rename" attribute
// matching the struct name, but in kebab-case, e.g. ConfigurationFiles -> "configuration-files".
// This lets it match the datastore name.
// Objects that live inside those top-level objects, e.g. Service lives in Services, should have
// rename="" so they don't add an extra prefix to the datastore path that doesn't actually exist.
// This is important because we have APIs that can return those sub-structures directly.

pub type Services = HashMap<String, Service>;

#[model(add_option = false, rename = "")]
struct Service {
    #[rand_derive(custom)]
    configuration_files: Vec<SingleLineString>,
    #[rand_derive(custom)]
    restart_commands: Vec<String>,
}

impl TestDataProviderForService for Service {
    fn generate_configuration_files<R: rand::Rng + ?Sized>(rng: &mut R) -> Vec<SingleLineString> {
        vec![]
    }

    fn generate_restart_commands<R: rand::Rng + ?Sized>(rng: &mut R) -> Vec<String> {
        vec![]
    }
}

pub type ConfigurationFiles = HashMap<String, ConfigurationFile>;

#[model(add_option = false, rename = "")]
struct ConfigurationFile {
    path: SingleLineString,
    template_path: SingleLineString,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[rand_derive(custom)]
    mode: Option<String>,
}

impl TestDataProviderForConfigurationFile for ConfigurationFile {
    fn generate_mode<R: rand::Rng + ?Sized>(rng: &mut R) -> Option<String> {
        None
    }
}

///// Metadata

#[model(add_option = false, rename = "metadata")]
struct Metadata {
    key: SingleLineString,
    md: SingleLineString,
    #[rand_derive(custom)]
    val: toml::Value,
}

impl TestDataProviderForMetadata for Metadata {
    fn generate_val<R: rand::Rng + ?Sized>(rng: &mut R) -> toml::Value {
        toml::Value::String("Hello world".to_string())
    }
}

///// Bootstrap Containers

#[model]
struct BootstrapContainer {
    source: Url,
    mode: BootstrapContainerMode,
    user_data: ValidBase64,
    essential: bool,
}

///// PEM Certificates
#[model]
struct PemCertificate {
    data: PemCertificateString,
    trusted: bool,
}

///// OCI hooks
#[model]
struct OciHooks {
    log4j_hotpatch_enabled: bool,
}

impl RandoHashmap<OciDefaultsCapability, bool> {
    pub fn generate<R: rand::Rng + ?Sized>(
        rng: &mut R,
        min: usize,
        max: usize,
    ) -> HashMap<OciDefaultsCapability, bool> {
        (0..rng.gen_range(min..max))
            .map(|_| (OciDefaultsCapability::generate_random(), rng.gen()))
            .collect()
    }
}

randogen_hashmap!(OciDefaultsResourceLimitType, OciDefaultsResourceLimit);

///// OCI defaults specifies the default values that will be used in cri-base-json.
#[model]
struct OciDefaults {
    #[rand_derive(custom)]
    capabilities: HashMap<OciDefaultsCapability, bool>,
    #[rand_derive(custom)]
    resource_limits: HashMap<OciDefaultsResourceLimitType, OciDefaultsResourceLimit>,
}

impl TestDataProviderForOciDefaults for OciDefaults {
    fn generate_capabilities<R: rand::Rng + ?Sized>(
        rng: &mut R,
    ) -> HashMap<OciDefaultsCapability, bool> {
        let r = RandoHashmap::<OciDefaultsCapability, bool>::generate(rng, 2, 5);
        maybe_return(rng, r)
    }

    fn generate_resource_limits<R: rand::Rng + ?Sized>(
        rng: &mut R,
    ) -> HashMap<OciDefaultsResourceLimitType, OciDefaultsResourceLimit> {
        let r = RandoHashmap::<OciDefaultsResourceLimitType, OciDefaultsResourceLimit>::generate(
            rng, 2, 5,
        );
        maybe_return(rng, r)
    }
}

///// The hard and soft limit values for an OCI defaults resource limit.
#[model(add_option = false)]
#[derive(
    Copy,
    Clone,
    Debug,
    Deserialize,
    Serialize,
    Eq,
    Ord,
    PartialOrd,
    PartialEq,
    rand_derive2::RandGen,
)]
struct OciDefaultsResourceLimit {
    #[serde(deserialize_with = "deserialize_limit")]
    hard_limit: i64,
    #[serde(deserialize_with = "deserialize_limit")]
    soft_limit: i64,
}

#[model(add_option = false)]
struct Report {
    #[rand_derive(custom)]
    name: String,
    #[rand_derive(custom)]
    description: String,
}

impl TestDataProviderForReport for Report {
    fn generate_name<R: rand::Rng + ?Sized>(rng: &mut R) -> String {
        crate::rando_alphanumeric_constrained(rng, 5, 30)
    }

    fn generate_description<R: rand::Rng + ?Sized>(rng: &mut R) -> String {
        crate::rando_alphanumeric_constrained(rng, 5, 30)
    }
}
