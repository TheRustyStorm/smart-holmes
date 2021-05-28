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

    pub fn map_to_update(services: &[Service]) -> Update{
        Update{version: 1, services:services.to_vec()}
    }

    pub fn generate_new_update(update: &Update, services: &[Service]) -> Update {
        let version = update.version + 1;
        let mut rng = rand::thread_rng();
        let random_dice = rng.gen_range(0..6);
        let mut serviceset = update.services.clone();
        match random_dice {
            0 => {
                if serviceset.len() > 1 {
                    let remove_index = rng.gen_range(0..serviceset.len());
                    serviceset.remove(remove_index);
                }
                return Update::new(version, serviceset);
            }
            1 => {
                let new_service_id = rng.gen_range(0..services.len());
                let mut has_service = false;
                for service in &serviceset {
                    if service.id == new_service_id {
                        has_service = true;
                    }
                }
                if !has_service {
                    serviceset.push(Service::new(new_service_id));
                }
                return Update::new(version, serviceset);
            }
            _ => (),
        }
        Update::new(version, serviceset)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_update(){
        assert_eq!(1,1);
    }
}