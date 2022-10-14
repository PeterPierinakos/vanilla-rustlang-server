use std::{collections::HashMap, fmt::Display};

pub fn map_as_json_str<T1: Display, T2: Display>(mp: HashMap<T1, T2>) -> String {
    let mut json = '{'.to_string();

    for (key, val) in mp {
        json.push_str(&format!("\"{key}\": \"{val}\","));
    }

    if json.len() > 1 {
        json.pop();
    }

    json.push('}');

    json
}

pub fn vec_as_json_str<T: Display>(vec: Vec<T>) -> String {
    let mut json = '['.to_string();

    for item in vec {
        json.push_str(&format!("\"{item}\","));
    }

    if json.len() > 1 {
        json.pop();
    }

    json.push(']');

    json
}

pub fn build_html(head: Vec<&str>, body: Vec<&str>) -> String {
    let mut doc = String::new();

    doc.push_str("<!DOCTYPE html>");
    doc.push_str("<html>");
    doc.push_str("<head>");

    for item in head {
        doc.push_str(item);
    }

    doc.push_str("</head>");

    doc.push_str("<body>");

    for item in body {
        doc.push_str(item);
    }

    doc.push_str("</body>");

    doc.push_str("</html>");

    doc
}
