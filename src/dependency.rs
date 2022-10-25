use super::device::Device;
use super::service::Service;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Dependency {
    pub device_indices: Vec<usize>,
    pub services: Vec<Service>,
    pub index: usize,
}

impl Dependency {

    #[must_use]
    pub fn new(device_indices: Vec<usize>, services: Vec<Service>, index: usize) -> Self {
        Self {
            device_indices,
            services,
            index,
        }
    }

    #[must_use]
    pub fn is_fullfilled_with_services(&self, services: &HashSet<Service>) -> bool {
        for service in &self.services {
            if services.get(service) == None {
                return false;
            }
        }
        true
    }

    #[must_use]
    pub fn number_active_devices(&self, devices: &[Device]) -> usize{
        self.device_indices
            .iter()
            .map(|index| devices.get(*index).unwrap())
            .filter(|device| device.is_active)
            .count()
    }

    #[must_use]
    pub fn number_inactive_devices(&self, devices: &[Device]) -> usize{
        self.device_indices
            .iter()
            .map(|index| devices.get(*index).unwrap())
            .filter(|device| !device.is_active)
            .count()
    }

    ///
    /// # Panics
    /// Panics if the device index that the dependency stores is out of the range of devices array
    /// 
    #[must_use]
    pub fn is_fullfilled(&self, devices: &[Device]) -> bool {
        for service in &self.services {
            let mut is_present = false;
            for index in &self.device_indices {
                match devices.get(*index){
                    Some(device) => {
                        if device.is_active{
                            for available_service in &device.services {
                                if *available_service == *service {
                                    is_present = true;
                                }
                            }
                        }
                    },
                    None => {
                        panic!("Couldn't get device with index {} ", index);
                    }
                }
            }
            if !is_present {
                return false;
            }
        }
        true
    }
}

impl fmt::Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\n Required: {:?}\n Given: {:#?}",
            self.services, self.device_indices
        )
    }
}
