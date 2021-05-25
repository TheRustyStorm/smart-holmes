use super::service::Service;
use super::update::Update;

#[derive(Clone,Debug)]
pub struct Device{
    pub services: Vec<Service>,
    pub updates: Vec<Update>
}

impl Device{
    pub fn new(services: Vec<Service>, updates: Vec<Update>) -> Device{
        Device{services, updates}
    }

}