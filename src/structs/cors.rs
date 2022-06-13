use crate::enums::methods::HttpRequestMethod;
use std::collections::HashSet;

pub struct Cors {
    allowed_methods: HashSet<HttpRequestMethod>,
}

#[allow(dead_code)]
impl Cors {
    pub fn new(allowed_methods: HashSet<HttpRequestMethod>) -> Cors {
        Cors {
            allowed_methods: allowed_methods,
        }
    }

    pub fn method_is_allowed(&self, buf: &String) -> bool {
        for method in self.allowed_methods.iter() {
            if buf.starts_with(&method.to_string()) {
                return true;
            }
        }
        false
    }
}
