use crate::device::Device;
use crate::smart_home::SmartHome;
use crate::update::Update;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Subsystem {
    pub devices: Vec<Device>,
}

#[derive(Debug, Clone)]
pub struct ConfigurationState {
    pub device_id: usize,
    pub updates: Vec<Update>,
}

impl fmt::Display for Subsystem {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} ", self.devices.len()).unwrap();
        for device in &self.devices {
            write!(f, "{} ", device.id).unwrap();
        }
        Ok(())
    }
}

impl Subsystem {
    fn new(devices: Vec<Device>) -> Self {
        Self { devices }
    }

    #[must_use]
    pub fn get_dependency_hashmap(smart_home: &SmartHome) -> HashMap<usize, Vec<usize>> {
        let mut dependencies_hashmap = HashMap::new();
        for dependency in &smart_home.dependencies {
            let mut devices = Vec::new();
            for device_index in &dependency.device_indices {
                let device = smart_home.get_device(*device_index);
                devices.push(device.id);
            }
            dependencies_hashmap.insert(dependency.index, devices);
        }
        dependencies_hashmap
    }

    fn color_devices(smart_home: &mut SmartHome) {
        let hashmap = Self::get_dependency_hashmap(smart_home);
        let mut has_changed;
        loop {
            has_changed = false;

            for devices in hashmap.values() {
                let mut min_of_dependency = usize::MAX;
                for device_index in devices {
                    let device = smart_home.get_device_mut(*device_index);
                    if min_of_dependency > device.color {
                        min_of_dependency = device.color;
                    }
                }
                for device_index in devices {
                    let device = smart_home.get_device_mut(*device_index);
                    if device.color > min_of_dependency {
                        device.color = min_of_dependency;
                        has_changed = true;
                    }
                }
            }
            if !has_changed {
                break;
            }
        }
    }

    pub fn find_subsystems(smart_home: &mut SmartHome) -> Vec<Self> {
        let mut subsystems: Vec<Self> = Vec::new();
        Self::color_devices(smart_home);
        let mut devices = smart_home.devices.clone();
        devices.sort();
        let sorted_devices = devices;
        let mut color;
        let mut index = 0;
        while index < sorted_devices.len() {
            color = sorted_devices[index].color;
            let mut color_vec = Vec::new();
            while index < sorted_devices.len() && color == sorted_devices[index].color {
                color_vec.push(sorted_devices[index].clone());
                index += 1;
            }
            subsystems.push(Self::new(color_vec));
        }
        subsystems
    }
}

#[cfg(test)]
mod tests {
    use crate::smart_home::*;
    use crate::subsystem::*;

    fn setup_hashmap() -> HashMap<usize, Vec<usize>> {
        let service_config = ServiceConfig {
            amount_services: 10,
        };
        let device_config = DeviceConfig {
            amount_devices: 10,
            services_per_device: 3,
        };
        let dependency_config = DependencyConfig {
            amount_dependencies: 5,
            device_per_dependency: 2,
            service_per_dependency: 2,
        };
        let update_config = UpdateConfig { amount_updates: 6 };

        let smart_home_config = Config::new(
            service_config,
            device_config,
            dependency_config,
            update_config,
        );
        let smart_home = SmartHome::new(smart_home_config);
        let hashmap = Subsystem::get_dependency_hashmap(&smart_home);
        hashmap
    }

    #[test]
    fn test_hashmap() {
        let hashmap = setup_hashmap();
        assert_eq!(5, hashmap.keys().len());
    }

    #[test]
    fn test_entries() {
        let _ = setup_hashmap();
    }
}
