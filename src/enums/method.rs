use std::fmt;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Method {
    GET,
    HEAD,
    OPTIONS,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
