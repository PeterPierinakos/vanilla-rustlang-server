pub struct JSONBuilder {
    pub json: String,
}

impl JSONBuilder {
    pub fn new() -> Self {
        let json = '{'.to_string();

        Self {
            json,
        }
    }

    pub fn add_pair(&mut self, key: String, val: String) {
        // If there are already existing pairs, then:
        if self.json.len() > 1 {
            self.json.push(',');
        }

        self.json.push_str(format!("\"{key}\": \"{val}\"").as_str());
    }

    pub fn build(&mut self) -> String {
        self.json.push('}'); 
        self.json.clone()
    }
}
