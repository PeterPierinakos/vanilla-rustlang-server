#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum HttpProtocolVersion {
    OneDotOne,
    Two,
}

impl<'a> Into<&'a str> for HttpProtocolVersion {
    fn into(self) -> &'a str {
        match self {
            Self::OneDotOne => "HTTP/1.1",
            _ => "HTTP/2",
        }
    }
}
