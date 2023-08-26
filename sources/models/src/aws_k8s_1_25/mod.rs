use model_derive::model;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{modeled_types::Identifier, rando_alphanumeric_constrained};
use crate::{
    AwsSettings, BootSettings, BootstrapContainer, CloudFormationSettings, DnsSettings,
    HostContainer, KernelSettings, MetricsSettings, NetworkSettings, NtpSettings, OciHooks,
    PemCertificate, RegistrySettings, UpdatesSettings,
};

// Note: we have to use 'rename' here because the top-level Settings structure is the only one
// that uses its name in serialization; internal structures use the field name that points to it
#[model(rename = "settings", impl_default = true)]
struct Settings {
    #[rand_derive(custom)]
    motd: String,
    updates: UpdatesSettings,
    #[rand_derive(custom)]
    host_containers: HashMap<Identifier, HostContainer>,
    #[rand_derive(custom)]
    bootstrap_containers: HashMap<Identifier, BootstrapContainer>,
    ntp: NtpSettings,
    network: NetworkSettings,
    kernel: KernelSettings,
    boot: BootSettings,
    aws: AwsSettings,
    metrics: MetricsSettings,
    #[rand_derive(custom)]
    pki: HashMap<Identifier, PemCertificate>,
    container_registry: RegistrySettings,
    oci_hooks: OciHooks,
    cloudformation: CloudFormationSettings,
    dns: DnsSettings,
}

impl TestDataProviderForSettings for Settings {
    fn generate_motd<R: rand::Rng + ?Sized>(rng: &mut R) -> Option<String> {
        let r = crate::rando_alphanumeric_constrained(rng, 5, 60);
        crate::maybe_return(rng, r)
    }

    fn generate_host_containers<R: rand::Rng + ?Sized>(
        rng: &mut R,
    ) -> Option<HashMap<Identifier, HostContainer>> {
        let r = crate::RandoHashmap::<Identifier, HostContainer>::generate(rng, 5, 10);
        crate::maybe_return(rng, r)
    }

    fn generate_bootstrap_containers<R: rand::Rng + ?Sized>(
        rng: &mut R,
    ) -> Option<HashMap<Identifier, BootstrapContainer>> {
        let r = crate::RandoHashmap::<Identifier, BootstrapContainer>::generate(rng, 5, 10);
        crate::maybe_return(rng, r)
    }

    fn generate_pki<R: rand::Rng + ?Sized>(
        rng: &mut R,
    ) -> Option<HashMap<Identifier, PemCertificate>> {
        let r = crate::RandoHashmap::<Identifier, PemCertificate>::generate(rng, 5, 10);
        crate::maybe_return(rng, r)
    }
}
