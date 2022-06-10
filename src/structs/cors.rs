use crate::enums::methods::HttpRequestMethod;
use std::collections::HashSet;
use std::net::TcpStream;

pub struct Cors {
    allowed_origins: String,
    allowed_methods: HashSet<HttpRequestMethod>,
}

#[allow(dead_code)]
impl Cors {
    pub fn new(allowed_origins: String, allowed_methods: HashSet<HttpRequestMethod>) -> Cors {
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

    pub fn allowed_methods(&self) -> &HashSet<HttpRequestMethod> {
        &self.allowed_methods
    }
}
