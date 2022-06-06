/*

ABSOLUTE_STATIC_CONTENT_PATH: The absolute path for your HTML/CSS files.
ABSOLUTE_LOGS_PATH: The absolute path for the location to store the log files at runtime (you may keep it blank if you have set "SAVE_LOGS" to "false".)
PORT: The port used by the server.
ADDR: The address used by the server. IPv4 only. Use "127.0.0.1" for localhost.
SAVE_LOGS: Change to "false" if you don't want logs of the requests to be saved at runtime.

Tree example for ABSOLUTE_STATIC_CONTENT_PATH:
ABSOLUTE_STATIC_CONTENT_PATH/
    html/
    css/

So, inside the static folder you have provided there must be an "html" and "css" folder and you have to place the static files accordingly.
*/

pub const ABSOLUTE_STATIC_CONTENT_PATH: &str = "/var/www/static";
pub const ABSOLUTE_LOGS_PATH: &str = "/var/www/logs";
pub const ADDR: &str = "127.0.0.1";
pub const PORT: u32 = 80;
pub const SAVE_LOGS: bool = true;
