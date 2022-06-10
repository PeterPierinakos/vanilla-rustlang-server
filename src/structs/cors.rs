use std::collections::HashSet;
use crate::enums::method::Method;
use std::net::TcpStream;

pub struct Cors {
    allowed_origins: String,
    allowed_methods: HashSet<Method>,
}

#[allow(dead_code)]
impl Cors {
    pub fn new(allowed_origins: String, allowed_methods: HashSet<Method>) -> Cors {
        Cors {
            allowed_origins: allowed_origins,
            allowed_methods: allowed_methods,
        }
    }

    pub fn method_is_allowed(&self, buf: String) -> bool {
        for method in self.allowed_methods.iter() {
            if buf.starts_with(&method.to_string()) {
                return true;
            }
        }
        false
    }

    pub fn origin_is_allowed(&self, origin: String) -> bool {
        // pass
        true
    }

    pub fn allowed_methods(&self) -> &HashSet<Method> {
        &self.allowed_methods
    }
}
