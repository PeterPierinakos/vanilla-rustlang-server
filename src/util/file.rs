use crate::configuration::ABSOLUTE_STATIC_CONTENT_PATH;
use std::fs;
use std::fs::File;

pub fn get_file_extension(filename: &str) -> &str {
    let mut last_dot_index = 0;
    for (i, c) in filename.char_indices() {
        if c == '.' {
            last_dot_index = i;
        }
    }
    &filename[last_dot_index + 1..]
}

pub fn find_file(filename: &str) -> File {
    let url = [ABSOLUTE_STATIC_CONTENT_PATH, "/", filename].concat();

    let file =
        fs::File::open(&url).expect(format!("{filename} file doesn't exist ('{}')", &url).as_str());

    file
}

pub fn find_mime_type(file_extension: &str) -> &str {
    match file_extension {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        _ => "text/plain",
    }
}
