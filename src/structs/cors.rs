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

impl<'a> Cors<'a> {
    pub fn new(
        allowed_origins: Vec<&'a str>,
        allow_all_origins: bool,
        allowed_methods: Vec<&'a str>,
    ) -> std::io::Result<Self> {
        /*  Static assertions! If an invalid value is provided, this will fail to build rather than failing at runtime */
        #[allow(clippy::manual_assert)]
        let _: () = {
            if allowed_origins.is_empty() && !allow_all_origins {
                panic!("ALLOWED_ORIGINS is set to false and no origins are provided");
            }
            if allowed_methods.is_empty() {
                panic!("You have to specify 1 or more allowed methods (GET recommended");
            }
        };

        let all_methods = HashSet::from([
            "GET", "HEAD", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "PATCH",
        ]);

        Ok(Self {
            allowed_methods: {
                for method in &allowed_methods {
                    if !all_methods.contains(method) {
                        return Err(Error::new(
                            ErrorKind::Other,
                            "Invalid method specified in ALLOWED_METHODS ({method})",
                        ));
                    }
                }
                Some(allowed_methods)
            },
            allowed_origins: if allowed_origins.is_empty() {
                None
            } else {
                Some(allowed_origins)
            },
        })
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
