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
        amount_devices: 50,
        services_per_device: 5,
    };
    let dependency_config = DependencyConfig {
        amount_dependencies: 50,
        device_per_dependency: 2,
        service_per_dependency: 5,
    };
    let update_config = UpdateConfig { amount_updates: 50 };

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
    let mut sum_none = 0;
    let mut sum_smart = 0;
    let mut sum_all = 0;
    let mut sum_random = 0;
    for _ in 0..1000 {
        let mut smart_home = generate_smart_home();
        let amount_implicit_dependencies_by_user = 5;
        let mut user = User::new(&smart_home, amount_implicit_dependencies_by_user);
        let mut transition_matrix = TransitionMatrix::new(smart_home.devices.len());
        let mut current_device;
        for _ in 0..50 {
            user.choose_random_new_device();
            current_device = user.currently_used_device();
            for _ in 0..100 {
                user.step();
                let next_device = user.currently_used_device();
                if next_device != current_device {
                    transition_matrix.increase(current_device, next_device);
                }
                current_device = next_device;
            }
        }
        //transition_matrix.print();
        for row in 0..smart_home.devices.len() {
            let sum_of_row = transition_matrix.sum_at_row(row);
            if sum_of_row > 10 {
                for column in 0..smart_home.devices.len() {
                    let value = transition_matrix.get_at(row, column);
                    if value * 2 > sum_of_row {
                        println!("Detected implicit dependency from {} to {}", row, column);
                        let new_dependency = Dependency::new(
                            vec![row, column],
                            smart_home.get_device(column).services.clone(),
                            smart_home.dependencies.len(),
                        );
                        smart_home.dependencies.push(new_dependency);
                    }
                }
            }
        }

        println!(
            "{} working dependencies",
            smart_home.amount_fullfilled_dependencies()
        );
        let mut smart_home_all = smart_home.clone();
        println!("Updating all");
        smart_home_all.update_all();
        println!(
            "{} working dependencies \t Update Score: {}",
            smart_home_all.amount_fullfilled_dependencies(),
            smart_home_all.update_score()
        );

        let mut smart_home_random = smart_home.clone();
        println!("Updating random");
        smart_home_random.update_random();
        println!(
            "{} working dependencies \t Update Score: {}",
            smart_home_random.amount_fullfilled_dependencies(),
            smart_home_random.update_score()
        );
        sum_none += smart_home.update_score();
        let mut smart_home_smart = smart_home;
        println!("Updating Smart");
        smart_home_smart.update_smart();
        println!(
            "{} working dependencies \t Update Score: {}",
            smart_home_smart.amount_fullfilled_dependencies(),
            smart_home_smart.update_score()
        );

       
        sum_all += smart_home_all.update_score();
        sum_random += smart_home_random.update_score();
        sum_smart += smart_home_smart.update_score();
    }
    println!("{} {} {} {}", sum_none/1000, sum_all/1000, sum_random/1000, sum_smart/1000);
    // let systems = Subsystem::find_subsystems(&mut smart_home);
    // println!("{}", systems[0]);
    // Subsystem::find_configurations(systems[0].clone());
}
