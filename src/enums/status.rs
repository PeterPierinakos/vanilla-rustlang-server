use std::fmt::Debug;

#[derive(Debug)]
pub enum TestStatusCode {
    DirResponse,
    OK,
    NotFound,
    BadRequest,
    MethodNotAllowed,
    InternalServerError,
    CORSError,
}
