use super::service::Service;

#[derive(Clone, Debug)]
pub struct Update{
    version: usize,
    services: Vec<Service>
}