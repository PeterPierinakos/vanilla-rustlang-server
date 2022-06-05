#[derive(Debug)]
pub struct Config {
    pub addr: Option<String>,
    pub port: Option<i32>,
    pub debug_info: bool,
}
