use smart_holmes::smart_home::*;
use smart_holmes::subsystem::Subsystem;

fn main() {
    let service_config = ServiceConfig {
        amount_services: 30,
    };
    let device_config = DeviceConfig {
        amount_devices: 20,
        services_per_device: 3,
    };
    let dependency_config = DependencyConfig {
        amount_dependencies: 5,
        device_per_dependency: 3,
        service_per_dependency: 3,
    };
    let update_config = UpdateConfig { amount_updates: 3 };

    let smart_home_config = SmartHomeConfig::new(
        service_config,
        device_config,
        dependency_config,
        update_config,
    );
    let mut smart_home = SmartHome::new(smart_home_config);
    //smart_home.save(String::from("a.json"));
    println!("Smart Home Created");
    //println!("{:#?}",smart_home.dependencies);
    //println!("{:?}", Subsystem::get_dependency_hashmap(&smart_home));
    let systems = Subsystem::find_subsystems(&mut smart_home);
    println!("{}", systems[0]);
    Subsystem::find_configurations(systems[0].clone());
}
