use smart_holmes::dependency::Dependency;
use smart_holmes::smart_home::*;
use smart_holmes::transition_matrix::*;
use smart_holmes::user::User;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

struct PrintableResult{
    pub x: Vec<usize>,
    pub none: Vec<usize>,
    pub all: Vec<usize>,
    pub random: Vec<usize>,
    pub smart: Vec<usize>,
}

impl PrintableResult{
    fn print(&self){
        println!("Smart");
        for index in 0..self.x.len(){
            print!("({},{})",self.x[index],self.smart[index]);
        }
        println!();

        println!("All");
        for index in 0..self.x.len(){
            print!("({},{})",self.x[index],self.all[index]);
        }
        println!();
        println!("Random");
        for index in 0..self.x.len(){
            print!("({},{})",self.x[index],self.random[index]);
        }
        println!();
        println!("None");
        for index in 0..self.x.len(){
            print!("({},{})",self.x[index],self.none[index]);
        }
        println!();
    }
}

fn generate_smart_home(input: usize) -> SmartHome {
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
    let update_config = UpdateConfig { amount_updates: input };

    let smart_home_config = SmartHomeConfig::new(
        service_config,
        device_config,
        dependency_config,
        update_config,
    );
    let smart_home = SmartHome::new(smart_home_config);
    smart_home
}

fn main() {
    let mut x = Vec::new();
    let mut none = Vec::new();
    let mut all = Vec::new();
    let mut random = Vec::new();
    let mut smart = Vec::new();
    
    for input in (0..=100).step_by(5){
    let mut sum_none = 0;
    let mut sum_smart = 0;
    let mut sum_all = 0;
    let mut sum_random = 0;
    for _ in 0..1000 {
        let mut smart_home = generate_smart_home(input);
        sum_none += smart_home.update_score();
        
        let mut smart_home_all = smart_home.clone();
        smart_home_all.update_all();
        sum_all += smart_home_all.update_score();
        
        let mut smart_home_random = smart_home.clone();
        smart_home_random.update_random();
        sum_random += smart_home_random.update_score();
        
        let mut smart_home_smart = smart_home;
        smart_home_smart.update_smart();
        sum_smart += smart_home_smart.update_score();
    }
        
    x.push(input);
    smart.push(sum_smart/1000);
    all.push(sum_all/1000);
    random.push(sum_random/1000);
    none.push(sum_none/1000);
    println!("{} {} {} {} {}", input, sum_smart/1000, sum_all/1000, sum_random/1000, sum_none/1000);
}
let r = PrintableResult { x, none, all, random, smart};
r.print();
}
