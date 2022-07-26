/*

Read the documentation if you want to find out how any of these configuration variables work.

*/

use crate::http::HttpProtocolVersion;

pub const ABSOLUTE_STATIC_CONTENT_PATH: &str = "/var/www/static";
pub const ABSOLUTE_LOGS_PATH: &str = "/var/www/logs";

/* Production note: "ADDR" should be "0.0.0.0" when running inside a Docker container. */
pub const ADDR: &str = "0.0.0.0";
pub const PORT: u32 = 80;

/* Production note: Multithreaded mode currently doesn't support logs. */
pub const MULTITHREADING: bool = false;
pub const NUM_OF_THREADS: usize = 1;

pub const SAVE_LOGS: bool = true;

pub const HTTP_PROTOCOL_VERSION: HttpProtocolVersion = HttpProtocolVersion::OneDotOne;

/* Security configuration below */

pub const ALLOWED_METHODS: [&str; 1] = ["GET"];

/* Production note: You should allow all origins if you want everyone to access your page. If you set "ALLOW_ALL_ORIGINS" to true, you may keep "ALLOWED_ORIGINS" empty. */
pub const ALLOW_ALL_ORIGINS: bool = true;
pub const ALLOWED_ORIGINS: [&str; 0] = [];

/* Production note: turning this off is not recommended and only for debugging purposes */
pub const SECURITY_HEADERS: bool = true;
pub const ALLOW_IFRAMES: bool = false;

pub const APPEND_EXTRA_HEADERS: bool = true;
pub const EXTRA_HEADERS_SIZE: usize = 0;
/* Production note: "EXTRA_HEADERS_SIZE" should be used to change the number of extra headers for
 * convenience. */
// Example: ["ServerHost", "VanillaRustlangServer"]
pub const EXTRA_HEADERS: [[&str; 2]; EXTRA_HEADERS_SIZE] = [];

pub const ALLOW_DIRECTORY_LISTING: bool = true;
