/*

Read the documentation at /docs/configuration.md if you want to find out how any of these configuration variables work.

CAUTION: PATHS MUST NOT CONTAIN TRAILING SLASH

*/

use crate::http::HttpProtocolVersion;

// Start of general server configuration 

// You should provide this variable the absolute path (/absolute/path/to/static) which should contain all the static files you went the web server to serve. The web server will take care of serving the files, all you need to know is that you need to put the same path inside the $STATIC variable in setup.sh.
pub const ABSOLUTE_STATIC_CONTENT_PATH: &str = "/var/www/static";

// The absolute path to which the server request logs should be saved (/absolute/path/to/logs). Note that this setting can be ignored if MULTITHREADING is enabled because the server doesn't save logs outside of singlethreaded mode.
pub const ABSOLUTE_LOGS_PATH: &str = "/var/www/logs";

// Boolean for specifying whether you want the server to save request logs for each request or not.
pub const SAVE_LOGS: bool = true;

// If enabled, the web server will cache previously requested files from the clients by storing them into the computer's free memory and displaying them instead of doing a filesystem operation on every request which is costly. This can double the performance on extremely large files, but in most cases it shouldn't make that much of a difference. If enabled, once a specific file has been requested its contents will never be updated after being changed. File caching can use up much memory depending on the files' sizes.
pub const CACHE_FILES: bool = true;

// The IPv4 address the server should use (e.g. 0.0.0.0 or 127.0.0.1). It is recommended to change this to a loopback address like 127.0.0.1 during development.
// Production note: should be "0.0.0.0" when running inside a Docker container. 
pub const ADDR: &str = "0.0.0.0";
// The port that which be used by the server. The default port for HTTP is 80 and 443 for HTTPS. Change accordingly.
pub const PORT: u32 = 80;

// Boolean which should be set to false if you wish to not print license information for the software upon starting. Note that keeping it enabled is the least you can do to show respect to the contributors who spent their time on improving this project.
pub const PRINT_LICENSE_INFO_AT_START: bool = true;

// Boolean to specify whether you want the server to allow listing the contents of a directory whenever a user tries to access a directory and not a particular file.
pub const ALLOW_DIRECTORY_LISTING: bool = true;

// Boolean to specify whether the returned directory's contents should be JSON. If it is set to false, it will return HTML. Can be ignored if ALLOW_DIRECTORY_LISTING is set to false.
pub const FORMAT_DIRECTORY_LISTING_AS_JSON: bool = false;

// End of general server configuration 

// Start of multithreading configuration 

// Boolean used to specify whether you want the server to use multiple threads (workers) or not. Logs aren't supported when using this functionality.
// Production note: Multithreaded mode currently doesn't support logs. 
pub const MULTITHREADING: bool = false;
pub const NUM_OF_THREADS: usize = 1;

// End of multithreading configuration 

// Enum for specifying whether you want to use HTTP/1.1 or HTTP/2 protocol version.
pub const HTTP_PROTOCOL_VERSION: HttpProtocolVersion = HttpProtocolVersion::OneDotOne;

// Start of CORS configuration 

// Used to specify which HTTP methods you wish to allow. Must contain atleast one. Most of the time you only need the GET method to serve plain HTML.
pub const ALLOWED_METHODS: [&str; 1] = ["GET"];


// Boolean used to specify whether you want to allow external web servers (outside your local network) to fetch data from VRS. There shouldn't be any problem with keeping this on.
// Production note: You should allow all origins if you want everyone to access your page. If you set "ALLOW_ALL_ORIGINS" to true, you may keep "ALLOWED_ORIGINS" empty. 
pub const ALLOW_ALL_ORIGINS: bool = true;

// If you don't want to allow all origins as explained above, you may also just allow a few specific origins to scrap from the server. If you want to block all origins, set ALLOW_ALL_ORIGINS to false and leave the ALLOWED_ORIGINS slice empty.
pub const ALLOWED_ORIGINS: [&str; 0] = [];

// End of CORS configuration 

// Start of headers configuration 

// You should not turn this off. When enabled, requests are sent some additional HTTP headers in order to prevent common attacks such as clickjacking.
// Production note: turning this off is not recommended and only for debugging purposes 
pub const USE_SECURITY_HEADERS: bool = true;

// The "Time" header is a special HTTP header used by the back-end in order to tell the client when the HTTP request was processed. Disable if you do not need this extra functionality.
pub const USE_TIME_HEADER: bool = true;

// Enable this boolean if you want to allow other web apps to embed your website inside them. May make common attacks possible, not recommended to enable unless deemed necessary.
pub const ALLOW_IFRAMES: bool = false;

// If you wish to apply the EXTRA_HEADERS below to all of the server's responses, set this boolean to true.
pub const APPEND_EXTRA_HEADERS: bool = true;

// Additional headers you can specify which will be applied to every server response as long as APPEND_EXTRA_HEADERS is set to true.
// Production note: "EXTRA_HEADERS_SIZE" should be used to change the number of extra headers for convenience.
// Example: ["ServerHost", "VanillaRustlangServer"]
pub const EXTRA_HEADERS: [(&str, &str); 0] = [];

// End of headers configuration 
