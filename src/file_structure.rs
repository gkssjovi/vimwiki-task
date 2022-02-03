use std::path::Path;
use chrono::{Local, DateTime};

use crate::{options::Options, file_manager::FileManager, utils::get_filename};

struct Info {
    dt: DateTime<Local>,
    index: String,
}

pub struct FileStructure {
    info: Info,
    pub target_dir: String,
    pub target_filename: String,
    pub tasks_dir: String,
    pub tasks_dir_name: String,
    pub root_dir: String,
    pub root_dir_name: String,
    pub assets_dir: String,
    pub assets_dir_name: String,
    pub docs_dir: String,
    pub docs_dir_name: String,
    pub docs_assets_dir: String,
    pub task_filename: String,
    pub doc_filename: String,
}

impl FileStructure {
    pub fn new(options: &Options) -> Self {
        let dt = Local::now();
        let index = String::from("1");

        let target_dir = options.get_target_dir();
        let target_filename = options.get_target();

        let tasks_dir = format!("{}/tasks", target_dir);
        let tasks_dir_name = get_filename(&tasks_dir);

        let root_dir = Self::get_next_root_dir(&options, &tasks_dir, &dt, &index);
        let root_dir_name = String::from(Path::new(&root_dir).file_name().unwrap().to_str().unwrap());

        let docs_dir = format!("{}/docs", root_dir);
        let docs_dir_name = String::from(Path::new(&docs_dir).file_name().unwrap().to_str().unwrap());

        let assets_dir = format!("{}/assets", root_dir);
        let assets_dir_name = String::from(Path::new(&assets_dir).file_name().unwrap().to_str().unwrap());

        let docs_assets_dir = format!("{}/assets", docs_dir);
        let task_filename = format!("{}/index.md", root_dir);
        let doc_filename = format!("{}/index.md", docs_dir);

        Self {
            info: Info { 
                dt,
                index,
            },
            target_dir,
            target_filename,
            tasks_dir,
            tasks_dir_name,
            root_dir,
            root_dir_name,
            assets_dir,
            assets_dir_name,
            docs_dir,
            docs_dir_name,
            docs_assets_dir,
            task_filename,
            doc_filename,
        }
    }
    
    pub fn create(&self) {
        FileManager::mkdir(&self.tasks_dir, true);
        FileManager::mkdir(&self.root_dir, true);
        FileManager::mkdir(&self.assets_dir, true);
        FileManager::mkdir(&self.docs_dir, true);
        FileManager::mkdir(&self.docs_assets_dir, true);
    }
    
    pub fn get_date_time(&self) -> DateTime<Local> {
        self.info.dt
    }
    
    pub fn get_index(&self) -> String {
        self.info.index.clone()
    }

    fn get_next_root_dir(options: &Options, tasks_dir: &str, dt: &DateTime<Local>, index: &str) -> String {
        let mut root_dir = Self::get_root_dir_filename(tasks_dir, dt, index);
        
        if options.get_override() {
            return root_dir;
        }

        let mut index_number = index
            .parse::<u32>()
            .unwrap();

        while Path::new(&root_dir).exists() {
            root_dir = Self::get_root_dir_filename(tasks_dir, dt, &index_number.to_string());
            index_number = index_number + 1;
        }

        root_dir
    }
    
    fn get_root_dir_filename(tasks_dir: &str, dt: &DateTime<Local>, index: &str) -> String {
        return format!("{}/{}_{}", tasks_dir, dt.format("%Y_%m_%d").to_string(), index);
    }
}