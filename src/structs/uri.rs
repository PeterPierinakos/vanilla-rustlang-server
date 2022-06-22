use std::path::{Component, Path, PathBuf};

pub struct URI {
    uri: Option<String>,
    found_error: bool,
}

impl URI {
    pub fn new() -> URI {
        URI {
            uri: None,
            found_error: false,
        }
    }

    pub fn find(&mut self, headers_buffer: &str) {
        let mut uri = String::new();

        for (i, c) in headers_buffer.chars().enumerate() {
            // After the "GET" in the INFO header.
            if c == ' ' && i != 3 {
                break;
            }
            if i > 4 {
                uri.push(c);
            }
        }

        let path = std::path::Path::new(&uri);

        if !URI::path_is_valid(Path::new(&path)) {
            self.uri = None;
            self.found_error = true;
            return;
        }

        if uri.is_empty() {
            self.uri = Some("index.html".to_string());
            return;
        }

        self.uri = Some(uri);
    }

    pub fn get(&self) -> &Option<String> {
        &self.uri
    }

    pub fn path_is_valid(path: &Path) -> bool {
        let mut result = PathBuf::new();
        let components = path.components();

        for component in components {
            match component {
                Component::RootDir | Component::Prefix(_) => return false, // Should be unreachable
                Component::CurDir => {
                    if result.as_os_str().is_empty() {
                        // If you've already stripped the leading / from the requested path, this should no-op
                        result.push(Component::RootDir);
                    }
                }
                Component::ParentDir => {
                    if !result.pop() {
                        return false;
                    }
                }
                Component::Normal(p) => result.push(p),
            };
        }
        true
    }
}

impl Default for URI {
    fn default() -> Self {
        Self::new()
    }
}
