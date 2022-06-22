#[allow(dead_code)]
#[derive(PartialEq)]
pub enum HttpProtocolVersion {
    OneDotOne,
    Two,
}

impl Clone for HttpProtocolVersion {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for HttpProtocolVersion {}
