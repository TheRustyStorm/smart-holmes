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
    pub version: usize,
    pub is_active: bool,
}

impl Device {
    /// Create a new Device, using a Vec of services that the device offers, a Vec of updates that it has and its id. 
    /// color is a field which will be used for finding subsystems, by assigning a subsystem of devices the same color.
    #[must_use]
    pub fn new(services: Vec<Service>, updates: Vec<Update>, id: usize) -> Self {
        Self {
            services,
            updates,
            id,
            color: id,
            version: 1,
            is_active: true,
        }
    }

    /// Deduplicate updates that offer the same service sets
    /// Is not used in this simulation
    pub fn remove_irrelevant_updates(&mut self) {
        self.updates.dedup();
    }

    /// Update a device to the target version, if it is available
    pub fn update(&mut self, target_version: usize) {
        let target_update = self.updates.iter().find(|n| n.version == target_version);
        if let Some(target_update) = target_update {
            self.services = target_update.services.clone();
            self.version = target_update.version;
            self.updates = self
                .updates
                .clone()
                .into_iter()
                .filter(|n| n.version >= self.version)
                .collect();
        }
    }
}

impl PartialOrd for Device {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.color.partial_cmp(&other.color)
    }
}

impl Ord for Device {
    fn cmp(&self, other: &Self) -> Ordering {
        self.color.cmp(&other.color)
    }
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
    }
}

impl Eq for Device {}
