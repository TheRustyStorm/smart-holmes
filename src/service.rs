use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::fmt;

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy, Debug)]
pub struct Service {
    pub id: usize,
}

impl fmt::Display for Service {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} ", self.id)
    }
}

impl Service {
    pub fn new(id: usize) -> Service {
        Service { id }
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
