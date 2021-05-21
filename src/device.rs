use super::service::Service;
use super::update::Update;

#[derive(Clone,Debug)]
pub struct Device{
    services: Vec<Service>,
    updates: Vec<Update>
}

impl Device{
    pub fn new(services: Vec<Service>, updates: Vec<Update>) -> Device{
        Device{services, updates}
    }

    pub fn services(&self) -> &Vec<Service>{
        &self.services
    }
}