use smart_holmes::dependency::Dependency;
use smart_holmes::smart_home::*;
use smart_holmes::transition_matrix::*;
use smart_holmes::user::User;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn generate_smart_home() -> SmartHome {
    let service_config = ServiceConfig {
        amount_services: 50,
    };
    let device_config = DeviceConfig {
        amount_devices: 5,
        services_per_device: 5,
    };
    let dependency_config = DependencyConfig {
        amount_dependencies: 50,
        device_per_dependency: 2,
        service_per_dependency: 5,
    };
    let update_config = UpdateConfig { amount_updates: 60 };

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
    let amount_implicit_dependencies_by_user = 0;
    let mut user = User::new(&smart_home, amount_implicit_dependencies_by_user);
    println!("{:?}",user.model);
    let mut transition_matrix = TransitionMatrix::new(smart_home.devices.len());
    let mut current_device;
    user.choose_random_new_device();
    loop {
        current_device = user.currently_used_device();
        user.step();
        let next_device = user.currently_used_device();
        if next_device != current_device {
            transition_matrix.increase(current_device, next_device);
        }
        //println!("{}",current_device);
        current_device = next_device;
    }
}
