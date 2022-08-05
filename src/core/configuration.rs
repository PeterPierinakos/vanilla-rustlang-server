use crate::configuration::*;
use crate::http::HttpProtocolVersion;

pub struct Configuration<'a> {
    pub absolute_static_content_path: &'a str,
    pub absolute_logs_path: &'a str,
    pub save_logs: bool,
    pub addr: &'a str,
    pub port: u32,
    pub multithreading: bool,
    pub num_of_threads: usize,
    pub http_protocol_version: HttpProtocolVersion,
    pub allowed_methods: Vec<&'a str>,
    pub allow_all_origins: bool,
    pub allowed_origins: Vec<&'a str>,
    pub use_security_headers: bool,
    pub use_time_header: bool,
    pub allow_iframes: bool,
    pub append_extra_headers: bool,
    pub extra_headers: Vec<[&'a str; 2]>,
    pub allow_directory_listing: bool,
    pub print_license_info: bool,
}

// Cloning is required by the `Server`.
impl<'a> Clone for Configuration<'a> {
    fn clone(&self) -> Self {
        Self {
            absolute_static_content_path: self.absolute_static_content_path,
            absolute_logs_path: self.absolute_logs_path,
            save_logs: self.save_logs,
            addr: self.addr,
            port: self.port,
            multithreading: self.multithreading,
            num_of_threads: self.num_of_threads,
            http_protocol_version: self.http_protocol_version,
            allowed_methods: self.allowed_methods.clone(),
            allow_all_origins: self.allow_all_origins,
            allowed_origins: self.allowed_origins.clone(),
            use_security_headers: self.use_security_headers,
            allow_iframes: self.allow_iframes,
            append_extra_headers: self.append_extra_headers,
            extra_headers: self.extra_headers.clone(),
            allow_directory_listing: self.allow_directory_listing,
            print_license_info: self.print_license_info,
            use_time_header: self.use_time_header,
        }
    }
}

impl<'a> Configuration<'a> {
    /// Will automatically set the configuration according to the configuration variables set in
    /// `src/configuration.rs`.
    pub fn read_from_vars() -> Self {
        Self {
            absolute_static_content_path: ABSOLUTE_STATIC_CONTENT_PATH,
            absolute_logs_path: ABSOLUTE_LOGS_PATH,
            save_logs: SAVE_LOGS,
            addr: ADDR,
            port: PORT,
            multithreading: MULTITHREADING,
            num_of_threads: NUM_OF_THREADS,
            http_protocol_version: HTTP_PROTOCOL_VERSION,
            allowed_methods: ALLOWED_METHODS.to_vec(),
            allow_all_origins: ALLOW_ALL_ORIGINS,
            allowed_origins: ALLOWED_ORIGINS.to_vec(),
            use_security_headers: USE_SECURITY_HEADERS,
            allow_iframes: ALLOW_IFRAMES,
            append_extra_headers: APPEND_EXTRA_HEADERS,
            extra_headers: EXTRA_HEADERS.to_vec(),
            allow_directory_listing: ALLOW_DIRECTORY_LISTING,
            print_license_info: PRINT_LICENSE_INFO_AT_START,
            use_time_header: USE_TIME_HEADER,
        }
    }
}
