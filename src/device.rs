use super::service::Service;
use super::update::Update;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Device {
    pub services: Vec<Service>,
    pub updates: Vec<Update>,
    pub id: usize,
    pub color: usize,
}

impl Device {
    pub fn new(services: Vec<Service>, updates: Vec<Update>, id: usize) -> Device {
        Device {
            services,
            updates,
            id,
            color: id,
        }
    }

    pub fn remove_irrelevant_updates(&mut self) {
        self.updates.dedup();
    }
}

impl PartialOrd for Device {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.color.partial_cmp(&other.color)
    }
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
    }
}
