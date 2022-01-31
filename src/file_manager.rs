use std::{fs::{read_to_string, create_dir, File, OpenOptions}, path::Path, io::Write};

use colour::green_ln;


pub struct FileManager {
    path: String,
}

impl FileManager {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
    
    pub fn read(&self) -> String {
        let template_content = read_to_string(&self.path)
            .expect(&format!("Unable to read the file \"{}\"", self.path));
        return template_content;
    }
    
    pub fn write(path: &str, content: &str, append: bool, log: bool) {
        let message = format!("File {} was written.", path);
        if append {
            let mut file = OpenOptions::new()
                .append(true)
                .open(&path)
                .expect("Could not write the file.");

            file.write_all(content.as_bytes())
                .expect("Cannot write the file.");
            
            if log {
                green_ln!("{}", message);
            }
        } else {
            let mut file = File::create(path)
                .expect(&format!("Unable to create the {} file.", path));

            file.write_all(content.as_bytes())
                .expect(&format!("Unable to write the {} file.", path));
            
            if log {
                green_ln!("{}", message);
            }
        }
    }
    
    pub fn mkdir(dirname: &str, log: bool) {
        if !Path::new(dirname).exists() {
            create_dir(dirname)
                .expect(&format!("Unable to create the directory \"{dirname}\"."));
            
            if log {
                green_ln!("Directory {}/ was created.", dirname);
            }
        }
    }
}