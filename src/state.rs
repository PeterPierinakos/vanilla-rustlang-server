use crate::file::CachedFile;
use std::collections::HashMap;

#[derive(Clone)]
pub struct AppState {
    pub cached_files: Option<HashMap<String, CachedFile>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self { cached_files: None }
    }
}
