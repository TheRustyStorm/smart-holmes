use smart_holmes::smart_home::*;

fn generate_smart_home() -> SmartHome {
    let service_config = ServiceConfig {
        amount_services: 500,
    };
    let device_config = DeviceConfig {
        amount_devices: 5000,
        services_per_device: 10,
    };
    let dependency_config = DependencyConfig {
        amount_dependencies: 2500,
        device_per_dependency: 2,
        service_per_dependency: 4,
    };
    let update_config = UpdateConfig { amount_updates: 3 };

    let smart_home_config = SmartHomeConfig::new(
        service_config,
        device_config,
        dependency_config,
        update_config,
    );
    let smart_home = SmartHome::new(smart_home_config);
    //smart_home.save(String::from("a.json"));
    println!("Smart Home Created");
    //println!("{:#?}",smart_home.dependencies);
    //println!("{:?}", Subsystem::get_dependency_hashmap(&smart_home));
    smart_home
}

fn main() {
    let smart_home = generate_smart_home();

    println!(
        "{} working dependencies",
        smart_home.amount_fullfilled_dependencies()
    );

    let mut smart_home_all = smart_home.clone();
    smart_home_all.update_all();
    println!("Updating all");
    println!(
        "{} working dependencies \t Update Score: {}",
        smart_home_all.amount_fullfilled_dependencies(), smart_home_all.update_score()
    );

    let mut smart_home_random = smart_home.clone();
    smart_home_random.update_random();
    println!("Updating random");
    println!(
        "{} working dependencies \t Update Score: {}",
        smart_home_random.amount_fullfilled_dependencies(), smart_home_random.update_score()
    );

    let mut smart_home_smart = smart_home.clone();
    smart_home_smart.update_smart();
    println!("Updating Smart");
    println!(
        "{} working dependencies \t Update Score: {}",
        smart_home_smart.amount_fullfilled_dependencies(), smart_home_smart.update_score()
    );

    // let systems = Subsystem::find_subsystems(&mut smart_home);
    // println!("{}", systems[0]);
    // Subsystem::find_configurations(systems[0].clone());
}
