

#[derive(Clone,Copy,Debug)]
pub struct Service{
    pub id: usize
}

impl Service{
    pub fn new(id: usize) -> Service{
        Service{id}
    }
}