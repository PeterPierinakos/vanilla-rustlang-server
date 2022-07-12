use crate::enums::http::HttpProtocolVersion;

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
    pub security_headers: bool,
    pub allow_iframes: bool,
    pub append_extra_headers: bool,
    pub extra_headers: Vec<[&'a str; 2]>,
    pub allow_directory_listing: bool, 
}

/* yuck */

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
            security_headers: self.security_headers,
            allow_iframes: self.allow_iframes,
            append_extra_headers: self.append_extra_headers,
            extra_headers: self.extra_headers.clone(),
            allow_directory_listing: self.allow_directory_listing,
        }
    }
}
