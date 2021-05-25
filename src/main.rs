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
    let amount_services = 20;
    let amount_devices = 20;
    let amount_dependencies = 5;
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
        let mut updates: Vec<Update> = Vec::new();
        let update = Update::map_to_update(&sample);
        updates.push(update);
        for i in 0..6{
            updates.push(Update::generate_new_update(&updates[i], &services));
        }
        let device = Device::new(sample, updates);
        println!("{:#?}",device);
        devices.push(device);
        
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
