mod service;
mod device;
mod update;
mod dependency;
mod smart_home;
use smart_home::*;

fn main() {
    let service_config = ServiceConfig{amount_services: 20};
    let device_config = DeviceConfig{amount_devices: 20, services_per_device: 3};
    let dependency_config = DependencyConfig{amount_dependencies: 10, device_per_dependency: 2, service_per_dependency: 2};
    let update_config = UpdateConfig{amount_updates: 6};

    let smart_home_config = SmartHomeConfig::new(service_config, device_config, dependency_config, update_config);
    let _smart_home = SmartHome::new(smart_home_config);
    //println!("{:?}",smart_home);
}
