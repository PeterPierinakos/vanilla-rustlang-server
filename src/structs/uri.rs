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

    pub fn find(&mut self, headers_buffer: &String) {
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

        println!("{}", uri);

        let path = std::path::Path::new(&uri);
        let mut components = path.components().peekable();

        if let Some(first) = components.peek() {
            if !matches!(first, std::path::Component::Normal(_)) {
                self.uri = None;
                self.found_error = true;
                return;
            }
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
}
