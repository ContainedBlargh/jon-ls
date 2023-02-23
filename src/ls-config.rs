extern crate argparse;
extern crate cli_select;
extern crate colored;
extern crate glob;

use argparse::{ArgumentParser, Store, StoreTrue};

#[macro_use]
mod config;

mod functions;

use config::*;
use std::collections::HashMap;
use PrettyColor::*;
use PrettyPos::*;
use PrettySelector::*;
use glob::glob;

use cli_select::Select;

fn edit_config(config: PrettyConfig) -> PrettyConfig {
    let mut mut_config = config.clone();
    println!("{:?}", &mut_config);
    let actions = vec!["add rule", "edit rule", "remove rule", "test rule against path", "back"];
    loop {
        let mut select = Select::new(&actions);
        let selected_item = select.start().to_string();
        match selected_item.as_str() {
            "remove rule" => {
                
            },
            "edit rule" => {
    
            },
            "remove rule" => {

            },
            "test rule against path" => {

            },
            "back" => {
                break;
            },
            _ => println!("Uh, what?"),
        }
    }
    mut_config
}

use std::io::prelude::*;
use std::path::PathBuf;

fn main() {
    println!("Welcome to the pretty-ls config, here you can:");
    let actions = vec!["edit config", "show an example", "exit", "save"];
    loop {
        let mut config = get_config!();
        let mut select = Select::new(&actions);
        let selected_item = select.start().to_string();
        match selected_item.as_str() {
            "save" => {
                let config_path = get_config_path();
                println!("saving to {:?}...", config_path);
                let output = serde_json::to_string_pretty(&config).unwrap();
                std::fs::remove_file(config_path.clone());
                let mut file = std::fs::File::create(config_path).unwrap();
                file.write_all(output.as_bytes()).unwrap();
                config = get_config!();
            },
            "exit" => {
                break;
            },
            "edit config" => {
                config = edit_config(config);
            },
            "show an example" => {
                let mut config_dir = dirs::config_dir().unwrap();
                config_dir.push("*");
                let mut paths: Vec<PathBuf> = vec![];
                for path in glob(config_dir.to_str().unwrap()).unwrap().filter_map(Result::ok) {
                    paths.push(path);
                }
                functions::pretty_print_simple(paths, config);
            },
            _ => println!("Uh, what?")
        }
    }
}
