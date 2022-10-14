pub mod response_builder;
pub mod types;
pub mod utils;

use crate::status::StatusCode;
use std::collections::HashMap;
use std::fs::File;

pub type ErrorResponse = (HashMap<String, String>, StatusCode);
pub type OkResponse = (HashMap<String, String>, Option<String>, Option<File>);

pub type ServerResponse = Result<OkResponse, ErrorResponse>;
