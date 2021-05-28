

#[derive(Clone,Copy,Debug)]
pub struct Service{
    pub id: usize
}

impl Service{
    pub fn new(id: usize) -> Service{
        Service{id}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_service() {
        let _ = Service::new(3);
    }

    #[test]
    fn test_service_id() {
        let service = Service::new(3);
        assert_eq!(3, service.id);
    }
}