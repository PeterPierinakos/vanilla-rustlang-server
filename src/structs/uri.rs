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

        let mut dot_c = 0;

        for (i, c) in headers_buffer.chars().enumerate() {
            // After the "GET" in the INFO header.
            if c == ' ' && i != 3 {
                break;
            }
            if i > 4 {
                if c == '.' {
                    dot_c += 1;
                }
                uri.push(c);
            }
        }

        if dot_c > 1 {
            self.found_error = true;
            self.uri = None;
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
}
