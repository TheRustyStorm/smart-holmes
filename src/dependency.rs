use super::device::Device;
use super::service::Service;

use std::fmt;

#[derive(Debug)]
pub struct Dependency{
    pub devices: Vec<Device>,
    services: Vec<Service>,
    pub index: usize,
}

impl Dependency{
    pub fn new(devices: Vec<Device>, services: Vec<Service>, index: usize) -> Dependency{
        Dependency{ devices, services, index}
    }

    pub fn is_fullfilled(&self) -> bool{
        for service in &self.services{
            let mut is_present = false;
            for device in &self.devices{
                for available_service in &device.services{
                    if available_service.id == service.id {
                        is_present = true;
                    }
                }
            }
            if !is_present{
                return false;
            }
        }
        true
    }
}

impl fmt::Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n Required: {:?}\n Given: {:#?}", self.is_fullfilled(), self.services, self.devices)   
    }
}