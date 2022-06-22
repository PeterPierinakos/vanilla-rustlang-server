pub struct HTMLBuilder<'a> {
    head: Vec<&'a str>,
    body: Vec<&'a str>,
}

impl<'a> HTMLBuilder<'a> {
    pub fn new() -> Self {
        let head = vec![
            r#"<meta charset="UTF-8" />"#,
            r#"<meta http-equiv="X-UA-Compatible" content="IE=edge" />"#,
            r#"<meta name="viewport" content="width=device-width, initial-scale=1.0" />"#,
        ];

        Self {
            head: head,
            body: vec![],
        }
    }

    pub fn add_to_head(&mut self, item: &'a str) {
        self.head.push(item);
    }

    pub fn add_to_body(&mut self, item: &'a str) {
        self.body.push(item);
    }

    pub fn get_len(&self) -> usize {
        self.body.len() + self.head.len()
    }

    pub fn construct(&self) -> String {
        let mut doc = String::new();

        doc.push_str("<!DOCTYPE html>");
        doc.push_str("<html>");
        doc.push_str("<head>");

        for item in self.head.iter() {
            doc.push_str(item);
        }

        doc.push_str("</head>");

        doc.push_str("<body>");

        for item in self.body.iter() {
            doc.push_str(item);
        }

        doc.push_str("</body>");

        doc.push_str("</html>");

        doc
    }
}
