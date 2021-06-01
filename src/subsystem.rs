use crate::device::Device;
use crate::smart_home::SmartHome;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Subsystem{
    pub devices: Vec<Device>,
}

impl Subsystem{
    fn new(devices: Vec<Device>) -> Subsystem{
        Subsystem{devices}
    }

    pub fn get_dependency_hashmap(smart_home: &SmartHome) -> HashMap<usize, Vec<usize>>{
        let mut dependencies = HashMap::new();
        for dependency in &smart_home.dependencies{
            let mut devices = Vec::new();
            for device in &dependency.devices{
                devices.push(device.id);
            }
            dependencies.insert(dependency.index, devices);
        }
        dependencies
    }

    fn subsystem_count(smart_home: &SmartHome) -> usize{
        let mut set = HashSet::new();
        for device in &smart_home.devices{
            set.insert(device.color);
        }
        set.len()
    }

    fn subsystem_color_set(smart_home: &SmartHome) -> HashSet<usize>{
        let mut set = HashSet::new();
        for device in &smart_home.devices{
            set.insert(device.color);
        }
        set
    }

    fn color_devices(smart_home: &mut SmartHome){
        let hashmap = Subsystem::get_dependency_hashmap(&smart_home);
        let mut has_changed;
        loop{
            has_changed = false;
            
            for devices in hashmap.values(){
                let mut min_of_dependency = usize::MAX;
                for device_index in devices{
                    let device = &mut smart_home.devices[*device_index];
                    if min_of_dependency > device.color{
                        min_of_dependency = device.color;
                    }
                }
                for device_index in devices{
                    let device = &mut smart_home.devices[*device_index];
                    if device.color > min_of_dependency{
                        device.color = min_of_dependency;
                        has_changed = true;
                    }
                }
            }
            if !has_changed{
                break;
            }
        }
        println!("{} subsystems found", Subsystem::subsystem_count(&smart_home));
    }

    pub fn find_subsystems(smart_home: &mut SmartHome) -> Vec<Subsystem>{
        let mut subsystems: Vec<Subsystem> = Vec::new();
        Subsystem::color_devices(smart_home);
        let subsystem_colors = Subsystem::subsystem_color_set(smart_home);
        for color in &subsystem_colors{
            let mut devices: Vec<Device> = Vec::new();
            for device in &smart_home.devices{
                if device.color == *color{
                    devices.push(device.clone());
                }
            }
            println!("Color {} \t with {} \telements", color, devices.len());
            subsystems.push(Subsystem::new(devices));
        }
        subsystems
    }
}

#[cfg(test)]
mod tests {
use crate::smart_home::*; 
use crate::update::*;
use crate::subsystem::*;

fn setup_hashmap() -> HashMap<usize, Vec<usize>>{
    let service_config = ServiceConfig{amount_services: 10};
    let device_config = DeviceConfig{amount_devices: 10, services_per_device: 3};
    let dependency_config = DependencyConfig{amount_dependencies: 5, device_per_dependency: 2, service_per_dependency: 2};
    let update_config = UpdateConfig{amount_updates: 6};

    let smart_home_config = SmartHomeConfig::new(service_config, device_config, dependency_config, update_config);
    let smart_home = SmartHome::new(smart_home_config);
    let hashmap = Subsystem::get_dependency_hashmap(&smart_home);
    hashmap
}

#[test]
fn test_hashmap(){
    let hashmap = setup_hashmap();
    assert_eq!(5, hashmap.keys().len());
}

#[test]
fn test_entries(){
    let hashmap = setup_hashmap();
}
}