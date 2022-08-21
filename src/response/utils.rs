use super::builder::ResponseBuilder;

/// Used to apply the extra headers specified in the configuration file.
///
/// Borrows an instance of `ResponseBuilder` in order to apply them.
pub fn apply_extra_headers<S: AsRef<str>>(response: &mut ResponseBuilder<S>, extra_headers: &Vec<[&str; 2]>) {
    for header in extra_headers.iter() {
        response.add_header(header[0].into(), header[1].into())
    }
}
