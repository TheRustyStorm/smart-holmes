use smart_holmes::smart_home::*;
use std::sync::{Arc, Mutex};


struct ResultStash {
    pub indices: Vec<usize>,
    pub none_strategy_measurements: Vec<usize>,
    pub all_strategy_measurements: Vec<usize>,
    pub random_strategy_measurements: Vec<usize>,
    pub smart_strategy_measurements: Vec<usize>,
}

impl ResultStash {
    fn new() -> ResultStash {
        ResultStash {
            indices: Vec::new(),
            none_strategy_measurements: Vec::new(),
            all_strategy_measurements: Vec::new(),
            random_strategy_measurements: Vec::new(),
            smart_strategy_measurements: Vec::new(),
        }
    }

    fn push_measurements(
        &mut self,
        index: usize,
        none_measurement: usize,
        all_measurement: usize,
        random_measurement: usize,
        smart_measurement: usize,
    ) {
        self.indices.push(index);
        self.smart_strategy_measurements.push(smart_measurement);
        self.all_strategy_measurements.push(all_measurement);
        self.random_strategy_measurements.push(random_measurement);
        self.none_strategy_measurements.push(none_measurement);
    }

    fn print(&self) {
        println!("Smart");
        for index in 0..self.indices.len() {
            print!("({},{})", self.indices[index], self.smart_strategy_measurements[index]);
        }
        println!();

        println!("All");
        for index in 0..self.indices.len() {
            print!("({},{})", self.indices[index], self.all_strategy_measurements[index]);
        }
        println!();
        println!("Random");
        for index in 0..self.indices.len() {
            print!("({},{})", self.indices[index], self.random_strategy_measurements[index]);
        }
        println!();
        println!("None");
        for index in 0..self.indices.len() {
            print!("({},{})", self.indices[index], self.none_strategy_measurements[index]);
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

fn main() {
    let mut result_stash = ResultStash::new();

    println!("Iterating over updates per device");
    let num_threads = 20;
    let num_measurements = 1000;
    // set min, max and set of the variable to iterate over
    for input in (0..=100).step_by(5) {
        println!("{}", input);
        
        //our threadsafe stores
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

        for thread_handles in thread_handles_vec {
            for thread_handle in thread_handles {
                match thread_handle.join() {
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
