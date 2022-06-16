use crate::configuration::{ALLOWED_ORIGINS, ALLOW_ALL_ORIGINS};
use crate::enums::http::HttpRequestMethod;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Cors<'a> {
    pub allowed_methods: HashSet<HttpRequestMethod>,
    pub allowed_origins: Option<Vec<&'a str>>,
}

#[allow(dead_code)]
impl Cors<'_> {
    pub fn new(allowed_methods: HashSet<HttpRequestMethod>) -> Self {
        if ALLOWED_ORIGINS.is_empty() && !ALLOW_ALL_ORIGINS {
            panic!("ALLOWED_ORIGINS is set to false and no origins are provided");
        }
        Self {
            allowed_methods: allowed_methods,
            allowed_origins: if ALLOWED_ORIGINS.is_empty() {
                None
            } else {
                Some(ALLOWED_ORIGINS.to_vec())
            },
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

    pub fn origin_is_allowed(&self, origin: &str) -> bool {
        if self.allowed_origins.is_none() {
            /* Because that means all origins are allowed. */
            return true;
        }
        for allowed_origin in self.allowed_origins.as_ref().unwrap().iter() {
            if *allowed_origin == origin {
                return true;
            }
        }
        false
    }
}
