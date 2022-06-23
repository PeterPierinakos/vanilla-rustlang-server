use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum TestStatusCode {
    DirResponse,
    OK,
    NotFound,
    BadRequest,
    MethodNotAllowed,
    InternalServerError,
    CORSError,
}
