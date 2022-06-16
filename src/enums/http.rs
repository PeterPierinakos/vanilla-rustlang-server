use std::fmt;

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum HttpProtocolVersion {
    OneDotOne,
    Two,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum HttpRequestMethod {
    GET,
    HEAD,
    OPTIONS,
}

impl fmt::Display for HttpRequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
