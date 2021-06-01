use super::service::Service;
use super::update::Update;

#[derive(Clone,Debug)]
pub struct Device{
    pub services: Vec<Service>,
    pub updates: Vec<Update>,
    pub id: usize,
    pub color: usize
}

impl Device{
    

    pub fn new(services: Vec<Service>, updates: Vec<Update>, id: usize) -> Device{
        Device{services, updates, id, color: id}
    }

}
