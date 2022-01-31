use std::collections::HashMap;

use chrono::{DateTime, Local};
use regex::Regex;


pub struct Variables {
    pub dt: DateTime<Local>,
    pub index: String,
    pub date_now: String,
    pub branch_name: String,
    pub title: String,
    pub target_name: String,
    pub assets_dir_name: String,
    pub docs_dir_name: String,
}

pub struct Template {
    content: String,
}

impl Template {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }
    
    pub fn render(&self, vars: HashMap<&str, String>) -> String {
        let mut render = String::from(&self.content);

        for (key, item) in vars.into_iter() {
            let re = Regex::new(&format!(r"(?P<item>\{{\b{key}\b\}})")).unwrap();
            render = re.replace_all(&render, item).to_string();
        }
        
        return render;
    }
}