mod bootstrap;
mod config;
mod utils;
mod template;
mod file_manager;
mod options;
mod file_structure;

use std::error::Error;

use bootstrap::{Bootstrap};
use clap::{load_yaml, App};
use config::{Config};
use options::Options;

fn main() -> Result<(), Box<dyn Error>> {
    let yaml = load_yaml!("options.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let options = Options::new(&matches);
    let config = Config::new(&options);
    Bootstrap::new(&config, &options).start();
    Ok(())
}
