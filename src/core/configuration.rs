use crate::configuration::*;
use crate::http::HttpProtocolVersion;
use std::collections::HashSet;

pub struct Configuration<'a> {
    pub absolute_static_content_path: &'a str,
    pub absolute_logs_path: &'a str,
    pub save_logs: bool,
    pub addr: &'a str,
    pub port: u32,
    pub multithreading: bool,
    pub num_of_threads: usize,
    pub http_protocol_version: HttpProtocolVersion,
    pub allowed_methods: HashSet<&'a str>,
    pub allow_all_origins: bool,
    pub allowed_origins: HashSet<&'a str>,
    pub use_security_headers: bool,
    pub use_time_header: bool,
    pub allow_iframes: bool,
    pub append_extra_headers: bool,
    pub extra_headers: Vec<[&'a str; 2]>,
    pub allow_directory_listing: bool,
    pub print_license_info: bool,
    pub cache_files: bool,
    pub format_directory_listing_as_json: bool,
}

// Cloning is required by the `Server`.
impl<'a> Clone for Configuration<'a> {
    fn clone(&self) -> Self {
        Self {
            allowed_methods: self.allowed_methods.clone(),
            allowed_origins: self.allowed_origins.clone(),
            extra_headers: self.extra_headers.clone(),
            ..*self
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
            allowed_methods: ALLOWED_METHODS.into(),
            allow_all_origins: ALLOW_ALL_ORIGINS,
            allowed_origins: ALLOWED_ORIGINS.into(),
            use_security_headers: USE_SECURITY_HEADERS,
            allow_iframes: ALLOW_IFRAMES,
            append_extra_headers: APPEND_EXTRA_HEADERS,
            extra_headers: EXTRA_HEADERS.into(),
            allow_directory_listing: ALLOW_DIRECTORY_LISTING,
            print_license_info: PRINT_LICENSE_INFO_AT_START,
            use_time_header: USE_TIME_HEADER,
            cache_files: CACHE_FILES,
            format_directory_listing_as_json: FORMAT_DIRECTORY_LISTING_AS_JSON,
        }
    }

    pub fn test_config() -> Self {
        Configuration {
            absolute_logs_path: ABSOLUTE_LOGS_PATH,
            absolute_static_content_path: "media",
            addr: "localhost",
            // Setting the port to 0 takes advantage of an OS behavior that
            // always uses a free port when assigned in this manner on all
            // major platforms.
            port: 0,
            allow_all_origins: false,
            allow_iframes: false,
            allowed_methods: HashSet::from(["GET"]),
            allowed_origins: HashSet::from(["localhost"]),
            save_logs: false,
            multithreading: false,
            num_of_threads: 1,
            http_protocol_version: HttpProtocolVersion::OneDotOne,
            use_security_headers: false,
            append_extra_headers: false,
            extra_headers: vec![],
            allow_directory_listing: true,
            print_license_info: false,
            use_time_header: false,
            cache_files: false,
            format_directory_listing_as_json: false,
        }
    }
}
