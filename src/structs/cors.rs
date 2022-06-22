use crate::configuration::{ALLOWED_METHODS, ALLOWED_ORIGINS, ALLOW_ALL_ORIGINS};

use std::{
    collections::HashSet,
    io::{Error, ErrorKind},
};

#[derive(Clone)]
pub struct Cors<'a> {
    // The size of both slices are unknown at compile time since its configurable so we have to use Vec
    pub allowed_methods: Option<Vec<&'a str>>,
    pub allowed_origins: Option<Vec<&'a str>>,
}

impl Cors<'_> {
    pub fn new() -> std::io::Result<Self> {
        // Static assertions! If an invalid value is provided, this will fail to build rather than failing at runtime
        #[allow(clippy::manual_assert)]
        const _: () = {
            if ALLOWED_ORIGINS.is_empty() && !ALLOW_ALL_ORIGINS {
                panic!("ALLOWED_ORIGINS is set to false and no origins are provided");
            }
            if ALLOWED_METHODS.is_empty() {
                panic!("You have to specify 1 or more allowed methods (GET recommended");
            }
        };

        let all_methods = HashSet::from([
            "GET", "HEAD", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "PATCH",
        ]);

        Ok(Self {
            allowed_methods: {
                for method in &ALLOWED_METHODS {
                    if !all_methods.contains(method) {
                        return Err(Error::new(
                            ErrorKind::Other,
                            "Invalid method specified in ALLOWED_METHODS ({method})",
                        ));
                    }
                }
                Some(ALLOWED_METHODS.into())
            },
            allowed_origins: if ALLOWED_ORIGINS.is_empty() {
                None
            } else {
                Some(ALLOWED_ORIGINS.into())
            },
        })
    }

    pub fn method_is_allowed(&self, buf: &str) -> bool {
        for method in self.allowed_methods.as_ref().unwrap().iter() {
            if buf.starts_with(method) {
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
