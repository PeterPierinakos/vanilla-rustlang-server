use std::fs;

/// The type of the content that will be sent back to the request.
///
/// `Self::File`: a response with a file from the filesystem.
/// `Self::Dir`: a response with the contents of a specific directory from the filesystem.
/// `Self::Fallback`: used when you want to return a status code page back to the user, usually
/// because of an error.
pub enum ResponseType<'a> {
    File(FileResponse<'a>),
    Dir(DirResponse),
    Fallback,
}

pub struct FileResponse<'a> {
    pub file_ext: &'a str,
    pub file_content: &'a str,
}

pub struct DirResponse {
    pub path_iterator: fs::ReadDir,
}
