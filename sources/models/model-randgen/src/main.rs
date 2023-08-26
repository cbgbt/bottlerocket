use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use rayon::prelude::*;
use serde::Serialize;
use std::fs::{create_dir_all, File};
use std::path::PathBuf;

struct NamedModel<T> {
    name: &'static str,
    _ghost: std::marker::PhantomData<T>,
}

impl<T: Send + Sync> NamedModel<T>
where
    Standard: Distribution<T>,
{
    fn new(name: &'static str) -> Self {
        Self {
            name,
            _ghost: std::marker::PhantomData,
        }
    }
}

trait RandoModel: Send + Sync {
    fn name(&self) -> &'static str;
    fn generate_model(&self) -> serde_json::Value;
}

impl<T> RandoModel for NamedModel<T>
where
    Standard: Distribution<T>,
    T: Serialize + Send + Sync,
{
    fn name(&self) -> &'static str {
        self.name
    }

    fn generate_model(&self) -> serde_json::Value {
        let mut rng = rand::thread_rng();
        let value: T = rng.gen();
        serde_json::to_value(value).unwrap()
    }
}

const NUM_GEN: usize = 5000;

fn main() {
    let outdir = PathBuf::from(std::env::args().nth(1).unwrap());

    #[rustfmt::skip]
    let models: Vec<Box<dyn RandoModel>> = vec![
        Box::new(NamedModel::<model::aws_dev::Settings>::new("aws-dev")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::aws_ecs_1::Settings>::new("aws-ecs-1")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::aws_ecs_1_nvidia::Settings>::new("aws-ecs-1-nvidia")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::aws_ecs_2::Settings>::new("aws-ecs-2")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::aws_ecs_2_nvidia::Settings>::new("aws-ecs-2-nvidia")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::aws_k8s_1_23::Settings>::new("aws-k8s-1.23")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::aws_k8s_1_24::Settings>::new("aws-k8s-1.24")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::aws_k8s_1_25::Settings>::new("aws-k8s-1.25")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::aws_k8s_1_26::Settings>::new("aws-k8s-1.26")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::aws_k8s_1_27::Settings>::new("aws-k8s-1.27")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::aws_k8s_1_23_nvidia::Settings>::new("aws-k8s-1.23-nvidia")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::aws_k8s_1_24_nvidia::Settings>::new("aws-k8s-1.24-nvidia")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::aws_k8s_1_25_nvidia::Settings>::new("aws-k8s-1.25-nvidia")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::aws_k8s_1_26_nvidia::Settings>::new("aws-k8s-1.26-nvidia")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::aws_k8s_1_27_nvidia::Settings>::new("aws-k8s-1.27-nvidia")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::metal_dev::Settings>::new("metal-dev")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::metal_k8s_1_23::Settings>::new("metal-k8s-1.23")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::metal_k8s_1_24::Settings>::new("metal-k8s-1.24")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::metal_k8s_1_25::Settings>::new("metal-k8s-1.25")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::metal_k8s_1_26::Settings>::new("metal-k8s-1.26")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::metal_k8s_1_27::Settings>::new("metal-k8s-1.27")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::vmware_dev::Settings>::new("vmware-dev")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::vmware_k8s_1_23::Settings>::new("vmware-k8s-1.23")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::vmware_k8s_1_24::Settings>::new("vmware-k8s-1.24")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::vmware_k8s_1_25::Settings>::new("vmware-k8s-1.25")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::vmware_k8s_1_26::Settings>::new("vmware-k8s-1.26")) as Box<dyn RandoModel>,
        Box::new(NamedModel::<model::vmware_k8s_1_27::Settings>::new("vmware-k8s-1.27")) as Box<dyn RandoModel>,
    ];

    for model in models {
        println!("Generating models for {}", model.name());
        let my_outdir = outdir.join(model.name());
        create_dir_all(&my_outdir).unwrap();
        (0..NUM_GEN).into_par_iter().for_each(|generated_num| {
            let value = model.generate_model();
            let filename = my_outdir.join(format!("{}_{:0>8}.json", model.name(), generated_num));

            serde_json::to_writer(File::create(filename).unwrap(), &value).unwrap();
        });
    }
}
