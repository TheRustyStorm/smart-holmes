use super::service::Service;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Update {
    version: usize,
    services: Vec<Service>,
}

impl Update {
    pub fn new(version: usize, services: Vec<Service>) -> Update {
        Update { version, services }
    }

    pub fn map_to_update(device_services: &[Service]) -> Update{
        Update{version: 1, services:device_services.to_vec()}
    }

    fn remove_servive(service_set: &mut Vec<Service>, service_id: usize){
        if service_set.len() > 1 { 
            service_set.remove(service_id);
        }
    }

    fn add_service(service_set: &mut Vec<Service>, new_service_id: usize){
        if service_set.iter().filter(|x| x.id == new_service_id).count() == 0{
            service_set.push(Service::new(new_service_id));
        }
    }

    pub fn generate_new_update(update: &Update, services: &[Service]) -> Update {
        let version = update.version + 1;
        let mut rng = rand::thread_rng();
        let random_dice = rng.gen_range(0..6); // 1/6 chance of removing a service, 1/6 chance of adding a service
        let mut service_set = update.services.clone();
        match random_dice {
            0 => {
                let remove_index = rng.gen_range(0..service_set.len());
                Update::remove_servive(&mut service_set, remove_index);
            }
            1 => {
                let new_service_id = rng.gen_range(0..services.len());
                Update::add_service(&mut service_set, new_service_id)   
            }
            _ => (),
        }
        Update::new(version, service_set)
    }
}


#[cfg(test)]
mod tests {
    
}