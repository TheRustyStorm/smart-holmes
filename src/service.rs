

#[derive(Clone,Copy,Debug)]
pub struct Service{
    id: usize
}

impl Service{
    pub fn new(id: usize) -> Service{
        Service{id}
    }

    pub fn id(&self) -> usize{
        self.id
    }
}