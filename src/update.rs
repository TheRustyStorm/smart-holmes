use super::service::Service;
use rand::Rng;
use serde::{Deserialize, Serialize};

/**
 * We assume that only one service is added or removed per update.
 * Therefore we have an Option for a single service, instead of a Vec.
 */
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Update {
    pub version: usize,
    pub services: Vec<Service>,
    pub added_services: Vec<Service>,
    pub removed_services: Vec<Service>,
}

impl PartialEq for Update {
    fn eq(&self, other: &Self) -> bool {
        let matching = self
            .services
            .iter()
            .zip(other.services.iter())
            .filter(|&(a, b)| a == b)
            .count();
        matching == self.services.len() && matching == other.services.len()
    }
}

impl Update {
    pub fn new(
        version: usize,
        services: Vec<Service>,
        added_services: Vec<Service>,
        removed_services: Vec<Service>,
    ) -> Update {
        Update {
            version,
            services,
            added_services,
            removed_services,
        }
    }

    pub fn map_to_update(device_services: &[Service]) -> Update {
        Update {
            version: 2,
            services: device_services.to_vec(),
            added_services: Vec::new(),
            removed_services: Vec::new(),
        }
    }

    fn remove_service(service_set: &mut Vec<Service>, service_id: usize) -> bool {
        if service_set.len() > 1 && service_id < service_set.len() {
            service_set.remove(service_id);
            return true;
        }
        false
    }

    fn add_service(service_set: &mut Vec<Service>, new_service_id: usize) {
        if service_set.iter().filter(|x| **x == new_service_id).count() == 0 {
            service_set.push(new_service_id);
        }
    }

    pub fn generate_new_update(update: &Update, services: &[Service]) -> Update {
        let version = update.version + 1;
        let mut rng = rand::thread_rng();
        let random_dice = rng.gen_range(0..5); 
        let mut service_set = update.services.clone();
        let mut removed_services = update.removed_services.clone();
        let mut added_services = update.added_services.clone();
        match random_dice {
            0 => {
                let remove_index = rng.gen_range(0..service_set.len());
                let removed_service_id = service_set[remove_index];
                if Update::remove_service(&mut service_set, remove_index) {
                    removed_services.push(removed_service_id);
                }
            }
            1 => {
                let new_service_id = rng.gen_range(0..services.len());
                added_services.push(new_service_id);
                Update::add_service(&mut service_set, new_service_id)
            }
            _ => (),
        }
        Update::new(version, service_set, added_services, removed_services)
    }
}

#[cfg(test)]
mod tests {
    use crate::smart_home::*;
    use crate::update::*;

    fn update_creator() -> Update {
        let service_config = ServiceConfig {
            amount_services: 10,
        };
        let device_config = DeviceConfig {
            amount_devices: 10,
            services_per_device: 3,
        };
        let update_config = UpdateConfig { amount_updates: 6 };
        let services = SmartHome::generate_services(&service_config);
        let device = SmartHome::generate_device(0, &device_config, &update_config, &services);

        Update::map_to_update(&device.services)
    }

    #[test]
    fn test_create_update() {
        let update = update_creator();
        assert_eq!(2, update.version);
    }

    #[test]
    fn test_add_new_service() {
        let mut update = Update {
            version: 1,
            services: vec![1, 2, 3],
            added_services: Vec::new(),
            removed_services: Vec::new(),
        };
        Update::add_service(&mut update.services, 4);
        assert_eq!(4, update.services.len());
    }

    #[test]
    fn test_add_existing_service() {
        let mut update = Update {
            version: 1,
            services: vec![1, 2, 3],
            added_services: Vec::new(),
            removed_services: Vec::new(),
        };
        Update::add_service(&mut update.services, 2);
        assert_eq!(3, update.services.len());
    }

    #[test]
    fn test_remove_existing_service() {
        let mut update = Update {
            version: 1,
            services: vec![1, 2, 3],
            added_services: Vec::new(),
            removed_services: Vec::new(),
        };
        let index = 2;
        Update::remove_service(&mut update.services, index);
        assert_eq!(2, update.services.len());
    }

    #[test]
    fn test_remove_not_existing_service() {
        let mut update = Update {
            version: 1,
            services: vec![1, 2, 3],
            added_services: Vec::new(),
            removed_services: Vec::new(),
        };
        let index = 3; //out of bounds
        Update::remove_service(&mut update.services, index);
        assert_eq!(3, update.services.len());
    }
}
