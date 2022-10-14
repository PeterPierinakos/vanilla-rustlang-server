use std::path::{Component, Path, PathBuf};

/// Finds the Uniform Resource Name in the request's buffer.
///
/// Returns `None` if an invalid path was found.
pub fn find_urn(buffer: &String) -> Option<String> {
    let mut uri = String::new();

    for (i, c) in buffer.chars().enumerate() {
        // After the "GET" in the INFO header.
        if c == ' ' && i != 3 {
            break;
        }
        if i > 4 {
            uri.push(c);
        }
    }

    let path = std::path::Path::new(&uri);

    if !path_is_valid(Path::new(&path)) {
        return None;
    }

    if uri.is_empty() {
        Some("index.html".to_string())
    } else {
        Some(uri)
    }
}

/// Invalidates the path in the URN to prevent path traversal attacks.
pub fn path_is_valid(path: &Path) -> bool {
    let mut result = PathBuf::new();
    let components = path.components();

    for component in components {
        match component {
            Component::Prefix(_) => return false, // Should be unreachable
            Component::RootDir => return false,   // Should be unreachable
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
