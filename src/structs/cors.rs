use crate::configuration::{ALLOWED_METHODS, ALLOWED_ORIGINS, ALLOW_ALL_ORIGINS};

use std::collections::HashSet;

#[derive(Clone)]
pub struct Cors<'a> {
    // The size of both slices are unknown at compile time since its configurable so we have to use Vec
    pub allowed_methods: Option<Vec<&'a str>>,
    pub allowed_origins: Option<Vec<&'a str>>,
}

impl Cors<'_> {
    pub fn new() -> Self {
        if ALLOWED_ORIGINS.is_empty() && !ALLOW_ALL_ORIGINS {
            panic!("ALLOWED_ORIGINS is set to false and no origins are provided");
        }
        if ALLOWED_METHODS.is_empty() {
            panic!("You have to specify 1 or more allowed methods (GET recommended");
        }

        let all_methods = HashSet::from([
            "GET", "HEAD", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "PATCH",
        ]);

        Self {
            allowed_methods: {
                for method in ALLOWED_METHODS.iter() {
                    if !all_methods.contains(method) {
                        panic!("Invalid method specified in ALLOWED_METHODS ({method})");
                    }
                }
                Some(ALLOWED_METHODS.into())
            },
            allowed_origins: if ALLOWED_ORIGINS.is_empty() {
                None
            } else {
                Some(ALLOWED_ORIGINS.into())
            },
        }
    }

    pub fn method_is_allowed(&self, buf: &String) -> bool {
        for method in self.allowed_methods.as_ref().unwrap().iter() {
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
