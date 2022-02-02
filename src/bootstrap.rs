use std::collections::HashMap;
use crate::config::Config;
use crate::file_structure::{FileStructure};
use crate::options::Options;
use crate::template::{Template, Variables};
use crate::file_manager::{FileManager};

pub struct Bootstrap<'a> {
    pub config: &'a Config<'a>,
    pub options: &'a Options<'a>,
}

impl<'a> Bootstrap<'a> {
    pub fn new(config: &'a Config, options: &'a Options) -> Self {
        Self {
            config,
            options,
        }
    }

    pub fn start(&self) {
        let file_structure = FileStructure::new(&self.options);
        file_structure.create();

        let dt = file_structure.get_date_time();
        let index = file_structure.get_index();
        let date_now = dt.format(&self.config.date_format).to_string();
        let branch_name = String::from(&self.config.branch_name);

        let title = Template::new(&self.config.task.title).render(HashMap::from([
            ("index", index.to_string()),
            ("date_now", date_now.to_string()),
            ("branch_name", branch_name.to_string()),
        ]));

        let vars = Variables {
            dt,
            index: index.clone(),
            date_now,
            branch_name,
            title: title.clone(),
            target_name: self.options.get_target_name(),
            docs_dir_name: file_structure.docs_dir_name,
            assets_dir_name: file_structure.assets_dir_name,
        };
        
        let title_line = self.get_title(&title, &file_structure.root_dir_name); 
        
        self.inser_title_line(&title_line);

        let template_content = self.render_template(&vars);
        FileManager::write(&file_structure.task_filename, &template_content, false, true);

        let doc_template_content = self.render_doc_template(&vars);
        FileManager::write(&file_structure.doc_filename, &doc_template_content, false, true);
    }
    
    fn get_variables(&self, vars: &Variables) -> HashMap<&str, String> {
        HashMap::from([
            ("title", vars.title.to_string()),
            ("branch_name", vars.branch_name.to_string()),
            ("date_time", vars.dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            ("date", vars.dt.format("%Y-%m-%d").to_string()),
            ("target_name", vars.target_name.to_string()),
            ("docs_dir_name", vars.docs_dir_name.to_string()),
            ("assets_dir_name", vars.assets_dir_name.to_string()),
        ])
    }
    
    // file: ./config/doc_template.md
    fn render_doc_template(&self, vars: &Variables) -> String {
        Template::new(&FileManager::new(&self.config.task.doc_template).read()).render(self.get_variables(&vars))
    }
    
    // file: ./config/template.md
    fn render_template(&self, vars: &Variables) -> String {
        Template::new(&FileManager::new(&self.config.task.template).read()).render(self.get_variables(&vars))
    }
    
    fn get_title(&self, title: &str, root_dir_name: &str) -> String {
        let title_line = format!("[{}](./{}/index.md)\n",title, root_dir_name);
        title_line
    }
    
    fn inser_title_line(&self, title_line: &str) {
        let target_filename = self.options.get_target();
        
        let target_line_number = self.options.get_line_number();
        if target_line_number > 0 {
            let mut target_content = FileManager::new(&target_filename).read();
            let mut line_number = 1;
            let lines = target_content.lines();

            let mut target_content_vector = Vec::new();
            for line in lines {
                target_content_vector.push(line);
                if line_number == target_line_number {
                    target_content_vector.push(&title_line.trim());
                }
                line_number += 1;
            }
            
            if target_line_number <= (target_content_vector.len() as u32) {
                target_content = target_content_vector.join("\n");
                FileManager::write(&target_filename, &target_content, false, true);
            } else {
                FileManager::write(&target_filename, &title_line, true, true);
            }
            
        } else {
            FileManager::write(&target_filename, &title_line, true, true);
        }
    }
}
