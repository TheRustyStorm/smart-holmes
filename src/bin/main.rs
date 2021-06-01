use smart_holmes::smart_home::*;
use smart_holmes::subsystem::Subsystem;

fn main() {
    let service_config = ServiceConfig{amount_services: 100};
    let device_config = DeviceConfig{amount_devices: 50, services_per_device: 5};
    let dependency_config = DependencyConfig{amount_dependencies: 20, device_per_dependency: 3, service_per_dependency: 3};
    let update_config = UpdateConfig{amount_updates: 6};

    let smart_home_config = SmartHomeConfig::new(service_config, device_config, dependency_config, update_config);
    let mut smart_home = SmartHome::new(smart_home_config);
    println!("Smart Home Created");
    //println!("{:#?}",smart_home.dependencies);
    //println!("{:?}", Subsystem::get_dependency_hashmap(&smart_home));
    Subsystem::find_subsystems(&mut smart_home);
}
