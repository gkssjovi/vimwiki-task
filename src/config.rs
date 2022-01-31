
use std::fs::{read_to_string};

use yaml_rust::{YamlLoader, Yaml};

use crate::{utils::absolute_path, options::Options};

pub struct  ConfigTask {
    pub title: String,
    pub template: String,
    pub doc_template: String,
}

pub struct Config<'a> {
    pub date_format: String,
    pub task: ConfigTask,
    pub options: &'a Options<'a>,
}

impl<'a> Config<'a> {
    pub fn new(options: &'a Options) -> Self {
        
        let doc = &(Self::read(&options)[0]);

        let config = Self {
            options,
            date_format: String::from(doc["date_format"].as_str()
                .expect("Configuration field \"date_format\" was not found.")),
            task: ConfigTask {
                title: String::from(doc["task"]["title"].as_str()
                    .expect("Configuration field \"task -> title\" was not found.")),
                template: String::from(absolute_path(
                    doc["task"]["template"]
                    .as_str()
                    .expect("Configuration field \"task -> template\" was not found.")
                ).unwrap().to_str().unwrap()),
                doc_template: String::from(absolute_path(
                    doc["task"]["doc_template"]
                    .as_str()
                    .expect("Configuration field \"task -> doc_template\" was not found.")
                ).unwrap().to_str().unwrap()),
            }
        };

        return config;
    }
    
    fn read(options: &'a Options) -> Vec<Yaml> {

        let config_path = options.get_config();

        let contents = read_to_string(&config_path)
            .expect("Unable to read the configuration file.");

        let docs = YamlLoader::load_from_str(&contents).unwrap();
        return docs;
    }
}