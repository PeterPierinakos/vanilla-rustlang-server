/*
Documentation in manual.html inside media/
*/

use crate::enums::http::HttpProtocolVersion;

pub const ABSOLUTE_STATIC_CONTENT_PATH: &str = "/var/www/static";
pub const ABSOLUTE_LOGS_PATH: &str = "/var/www/logs";

/* Production note: "ADDR" should be "0.0.0.0" when running inside a Docker container. */
pub const ADDR: &str = "0.0.0.0";

pub const PORT: u32 = 80;
pub const SAVE_LOGS: bool = true;

/* Production note: Multithreaded mode currently doesn't support logs. */
pub const MULTITHREADING: bool = false;

pub const HTTP_PROTOCOL_VERSION: HttpProtocolVersion = HttpProtocolVersion::OneDotOne;
