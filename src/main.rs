mod service;
mod device;
mod update;
mod dependency;
use service::Service;
use device::Device;
use update::Update;
use dependency::Dependency;
use rand::seq::SliceRandom;

fn main() {
    let amount_services = 100;
    let amount_devices = 5000;
    let amount_dependencies = 3;
    let device_per_dependency = 2;
    let service_per_dependency = 2;
    let mut services = Vec::new();
    let mut devices = Vec::new();
    let mut dependencies = Vec::new();
    for i in 0..amount_services{
        services.push(Service::new(i));
    }

    for _ in 0..amount_devices{
        let sample: Vec<_> = services
            .choose_multiple(&mut rand::thread_rng(), 3)
            .cloned()
            .collect();
        let updates: Vec<Update> = Vec::new();
        devices.push(Device::new(sample, updates));
    }

    while dependencies.len() < amount_dependencies{
        let dependency_devices: Vec<_> = devices
            .choose_multiple(&mut rand::thread_rng(), device_per_dependency)
            .cloned()
            .collect();
        let dependency_services: Vec<_> = services
            .choose_multiple(&mut rand::thread_rng(), service_per_dependency)
            .cloned()
            .collect();
        let dependency = Dependency::new(dependency_devices, dependency_services);
        if dependency.is_fullfilled(){
            println!("{}",dependency);
            dependencies.push(dependency);
        }
    }

}
