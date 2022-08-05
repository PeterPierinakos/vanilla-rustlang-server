use crate::configuration::ABSOLUTE_STATIC_CONTENT_PATH;
use crate::error::ServerError;
use std::fs::File;
use std::{fs, io};

pub fn get_file_extension(filename: &str) -> &str {
    let mut last_dot_index = 0;
    for (i, c) in filename.char_indices() {
        if c == '.' {
            last_dot_index = i;
        }
    }
    &filename[last_dot_index + 1..]
}

pub fn find_file(absolute_static_content_path: &str, filename: &str) -> Result<File, ServerError> {
    let url = [absolute_static_content_path, "/", filename].concat();

    let file = match fs::File::open(&url) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                return Err(ServerError::IOError(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("{filename} file doesn't exist ('{}'). Did you forget to run setup.sh script? Refer to the documentation for more information about configuration.", &url).as_str(),
                )))
            },
            _ => return Err(ServerError::IOError(e)),
        },
    };

    Ok(file)
}

pub fn find_mime_type(file_extension: &str) -> &str {
    match file_extension {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        _ => "text/plain",
    }
}
