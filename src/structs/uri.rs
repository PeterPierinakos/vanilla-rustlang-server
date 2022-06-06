pub struct URI {
    uri: Option<String>,
}

impl URI {
    pub fn new() -> URI {
        URI { uri: None }
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
