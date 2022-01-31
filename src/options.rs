use std::{path::Path, process};

use clap::ArgMatches;
use colour::red_ln;

use crate::utils::absolute_path;


pub struct Options<'a> {
    matches: &'a ArgMatches<'a>,
}

impl<'a> Options<'a> {

    pub fn new(matches: &'a ArgMatches) -> Self {
        Self {
            matches: &matches,
        }
    }
    
    pub fn get_target(&self) -> String {
        let result = self.matches.value_of("TARGET");
        
        let target = match result {
            Some(target) => target,
            None => {
                red_ln!("You must provide the target file arugment.");
                std::process::exit(1);
            }
        };
        
        if !Path::new(&target).exists() {
            red_ln!("The target file \"{}\" does not exits.", target);
            std::process::exit(1);
        }
        
        return absolute_path(target.to_string())
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
    }
    
    pub fn get_target_dir(&self) -> String {
        let target = self.get_target();
        let path = Path::new(&target)
            .parent()
            .unwrap();

        return path.as_os_str()
            .to_os_string()
            .into_string()
            .unwrap();
    }
    
    pub fn get_target_name(&self) -> String{
       let target = self.get_target();

       return String::from(Path::new(&target)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap());
    }
    
    pub fn get_line_number(&self) -> u32 {
        let result = self.matches
            .value_of("line-number")
            .unwrap_or("0");

        return result.parse::<u32>()
            .expect(&format!("invalid line-number argument. the value should be an integer and your's is \"{}\".", result));
    }

    pub fn get_override(&self) -> bool {
        let result = self.matches
            .value_of("override");

        match result {
           Some("true") => true,
           Some("false") => false,
           Some("1") => true,
           Some("0") => false,
           _ => false,
        }
    }
    
    pub fn get_config(&self) -> String {
        let result = self.matches
            .value_of("config")
            .unwrap_or("~/.config/vimwiki-task/config.yaml");
        
        let path = absolute_path(result).unwrap();

        match path.exists() {
            true => path.to_str().unwrap().to_string(),
            false => {
                red_ln!("The configuration file \"{}\" was not found.", path.to_str().unwrap());
               process::exit(1);
            },
        }
    }
}