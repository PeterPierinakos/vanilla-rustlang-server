/*

Read the documentation at /docs/configuration.md if you want to find out how any of these configuration variables work.

*/

use crate::http::HttpProtocolVersion;

/* Start of general server configuration */

pub const ABSOLUTE_STATIC_CONTENT_PATH: &str = "/var/www/static";
pub const ABSOLUTE_LOGS_PATH: &str = "/var/www/logs";
pub const SAVE_LOGS: bool = true;

pub const CACHE_FILES: bool = true;

/* Production note: "ADDR" should be "0.0.0.0" when running inside a Docker container. */
pub const ADDR: &str = "0.0.0.0";
pub const PORT: u32 = 80;

pub const PRINT_LICENSE_INFO_AT_START: bool = true;

pub const ALLOW_DIRECTORY_LISTING: bool = true;
pub const FORMAT_DIRECTORY_LISTING_AS_JSON: bool = false;

/* End of general server configuration */

/* Start of multithreading configuration */

/* Production note: Multithreaded mode currently doesn't support logs. */
pub const MULTITHREADING: bool = false;
pub const NUM_OF_THREADS: usize = 1;

/* End of multithreading configuration */

pub const HTTP_PROTOCOL_VERSION: HttpProtocolVersion = HttpProtocolVersion::OneDotOne;

/* Start of CORS configuration */

pub const ALLOWED_METHODS: [&str; 1] = ["GET"];

/* Production note: You should allow all origins if you want everyone to access your page. If you set "ALLOW_ALL_ORIGINS" to true, you may keep "ALLOWED_ORIGINS" empty. */
pub const ALLOW_ALL_ORIGINS: bool = true;
pub const ALLOWED_ORIGINS: [&str; 0] = [];

/* End of CORS configuration */

/* Start of headers configuration */

/* Production note: turning this off is not recommended and only for debugging purposes */
pub const USE_SECURITY_HEADERS: bool = true;
pub const USE_TIME_HEADER: bool = true;
pub const ALLOW_IFRAMES: bool = false;

pub const APPEND_EXTRA_HEADERS: bool = true;
/* Production note: "EXTRA_HEADERS_SIZE" should be used to change the number of extra headers for
 * convenience. */
// Example: ["ServerHost", "VanillaRustlangServer"]
pub const EXTRA_HEADERS: [(&str, &str); 0] = [];

/* End of headers configuration */
