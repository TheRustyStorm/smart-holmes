use smart_holmes::smart_home::*;
use std::sync::{Arc, Mutex};

#[allow(dead_code)]

struct ResultStash {
    pub x: Vec<usize>,
    pub none: Vec<usize>,
    pub all: Vec<usize>,
    pub random: Vec<usize>,
    pub smart: Vec<usize>,
}

impl ResultStash {
    #[allow(dead_code)]

    fn new() -> ResultStash {
        ResultStash {
            x: Vec::new(),
            none: Vec::new(),
            all: Vec::new(),
            random: Vec::new(),
            smart: Vec::new(),
        }
    }
    #[allow(dead_code)]

    fn push_measurements(
        &mut self,
        x: usize,
        none: usize,
        all: usize,
        random: usize,
        smart: usize,
    ) {
        self.x.push(x);
        self.smart.push(smart);
        self.all.push(all);
        self.random.push(random);
        self.none.push(none);
    }
    #[allow(dead_code)]

    fn print(&self) {
        println!("Smart");
        for index in 0..self.x.len() {
            print!("({},{})", self.x[index], self.smart[index]);
        }
        println!();

        println!("All");
        for index in 0..self.x.len() {
            print!("({},{})", self.x[index], self.all[index]);
        }
        println!();
        println!("Random");
        for index in 0..self.x.len() {
            print!("({},{})", self.x[index], self.random[index]);
        }
        println!();
        println!("None");
        for index in 0..self.x.len() {
            print!("({},{})", self.x[index], self.none[index]);
        }
        println!();
    }
}

/*
    Set variable that will be iterated over to input
*/
fn generate_smart_home(input: usize) -> SmartHome {
    let service_config = ServiceConfig {
        amount_services: 50, //default 50
    };
    let device_config = DeviceConfig {
        amount_devices: 50,     //default 50
        services_per_device: 5, //default 5
    };
    let dependency_config = DependencyConfig {
        amount_dependencies: 50,   //default 50
        device_per_dependency: 2,  //default 2
        service_per_dependency: 5, //default 5
    };
    let update_config = UpdateConfig {
        amount_updates: input,
    }; //default 5

    let smart_home_config = Config::new(
        service_config,
        device_config,
        dependency_config,
        update_config,
    );
    SmartHome::new(&smart_home_config)
}

#[allow(dead_code)]

fn experiment_updates() {
    let mut result_stash = ResultStash::new();

    let num_threads = 10;
    let num_measurements = 1000;
    // set min, max and set of the variable to iterate over
    for input in (0..=100).step_by(5) {
        println!("{}", input);
        #[allow(clippy::mutex_atomic)]
        let sum_none = Arc::new(Mutex::new(0));
        #[allow(clippy::mutex_atomic)]
        let sum_all = Arc::new(Mutex::new(0));
        #[allow(clippy::mutex_atomic)]
        let sum_random = Arc::new(Mutex::new(0));
        #[allow(clippy::mutex_atomic)]
        let sum_smart = Arc::new(Mutex::new(0));
        let mut thread_handles_vec = Vec::with_capacity(num_threads);

        for _ in 0..num_threads {
            let mut handles = Vec::with_capacity(num_measurements / num_threads);
            for _ in 0..num_measurements / num_threads {
                let m_none = Arc::clone(&sum_none);
                let m_all = Arc::clone(&sum_all);
                let m_random = Arc::clone(&sum_random);
                let m_smart = Arc::clone(&sum_smart);
                let handle = std::thread::spawn(move || {
                    let smart_home = generate_smart_home(input);
                    let mut none = m_none.lock().unwrap();
                    *none += smart_home.update_score();
                    drop(none);

                    let mut smart_home_all = smart_home.clone();
                    smart_home_all.update_all();
                    let mut all = m_all.lock().unwrap();
                    *all += smart_home_all.update_score();
                    drop(all);

                    let mut smart_home_random = smart_home.clone();
                    smart_home_random.update_random();
                    let mut random = m_random.lock().unwrap();
                    *random += smart_home_random.update_score();
                    drop(random);

                    let mut smart_home_smart = smart_home;
                    smart_home_smart.update_smart();
                    let mut smart = m_smart.lock().unwrap();
                    *smart += smart_home_smart.update_score();
                });
                handles.push(handle);
            }
            thread_handles_vec.push(handles);
        }

        for h in thread_handles_vec {
            for hh in h {
                match hh.join() {
                    Ok(_) => {}
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                }
            }
        }

        result_stash.push_measurements(
            input,
            *sum_none.lock().unwrap() / num_measurements,
            *sum_all.lock().unwrap() / num_measurements,
            *sum_random.lock().unwrap() / num_measurements,
            *sum_smart.lock().unwrap() / num_measurements,
        );
    }
    result_stash.print();
}

fn experiment_removed_devices() {
    let service_config = ServiceConfig {
        amount_services: 50, //default 50
    };
    let device_config = DeviceConfig {
        amount_devices: 50,     //default 50
        services_per_device: 5, //default 5
    };
    let dependency_config = DependencyConfig {
        amount_dependencies: 50,   //default 50
        device_per_dependency: 3,  //default 2
        service_per_dependency: 5, //default 5
    };
    let update_config = UpdateConfig { amount_updates: 5 }; //default 5

    let smart_home_config = Config::new(
        service_config,
        device_config,
        dependency_config,
        update_config,
    );

    let mut fulfilled_vec: Vec<usize> = vec![0; smart_home_config.device_config.amount_devices + 1];
    let mut links_mesh_vec: Vec<usize> = vec![0; smart_home_config.device_config.amount_devices + 1];
    let mut links_line_vec: Vec<usize> = vec![0; smart_home_config.device_config.amount_devices + 1];
    let num_repetitions = 1000;

    for _ in 0..num_repetitions {
        let mut smart_home = SmartHome::new(&smart_home_config);
        fulfilled_vec[0] += smart_home.amount_fullfilled_dependencies();
        links_mesh_vec[0] += smart_home.amount_links_between_devices_mesh();
        links_line_vec[0] += smart_home.amount_links_between_devices_line();
        for i in 0..smart_home.devices.len() {
            smart_home.devices[i].is_active = false;
            fulfilled_vec[i + 1 ] += smart_home.amount_fullfilled_dependencies();
            links_mesh_vec[i + 1] += smart_home.amount_links_between_devices_mesh();
            links_line_vec[i + 1] += smart_home.amount_links_between_devices_line();
        }
    }
    println!("Broken Devices \tFulfilled Dependencies \tLinks between Devices (M)\tLinks between Devices (L)");
    for i in 0..smart_home_config.device_config.amount_devices + 1 {
        println!(
            "{}\t\t{}\t\t\t{}\t\t\t\t{}",
            i,
            fulfilled_vec[i] / num_repetitions,
            links_mesh_vec[i] / num_repetitions,
            links_line_vec[i] / num_repetitions,
        );
    }
}

fn main() {
    experiment_removed_devices();
}
