pub mod builder;
pub mod types;
pub mod factory;
pub mod final_response;
pub mod htmlbuilder;
pub mod utils;

use crate::headers::Header;
use std::fs::File;
use crate::status::StatusCode;

pub type ErrorResponse = (Header, StatusCode);
pub type OkResponse = (Header, Option<String>, Option<File>);

/* Headers, Status Code, Response File */
pub type ServerResponse<'a> = Result<OkResponse, ErrorResponse>;
