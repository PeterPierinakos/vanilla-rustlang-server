// Contains some unit tests.

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use vrs::response::builder::ResponseBuilder;
    use vrs::response::utils::apply_extra_headers;

    #[test]
    fn test_apply_headers_works() {
        let mut response = ResponseBuilder::new();
        apply_extra_headers(
            &mut response,
            &vec![["ServerHost", "VanillaRustlangServer"]],
        );
        let headers = response.get_headers();
        assert_eq!(
            headers,
            HashMap::from([("ServerHost".into(), "VanillaRustlangServer".into())])
        );
    }
}
