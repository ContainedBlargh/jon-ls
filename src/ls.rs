extern crate argparse;
extern crate colored;
extern crate glob;
extern crate terminal_size;

mod functions;

#[macro_use]
mod config;

use argparse::{ArgumentParser, Store, StoreTrue};
use functions::*;
use glob::glob;
use std::path::PathBuf;

fn main() {
    let mut glob_path = String::from("*");
    let mut plain = false;
    let mut copyright = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("List files.");
        ap.refer(&mut glob_path).add_argument(
            "path",
            Store,
            "path in glob format that should have it's files listed.",
        );
        ap.refer(&mut plain).add_option(
            &["-p", "--plain"],
            StoreTrue,
            "List results without colors.",
        );
        ap.refer(&mut copyright).add_option(
            &["-c", "--copyright"],
            StoreTrue,
            "Show copyright notice.",
        );
        ap.parse_args_or_exit();
    }
    if copyright {
        println!("(c) Jon Voigt 2022 - This software can be used at your own risk, but for your own gain as well.");
    }
    if !glob_path.ends_with("*") {
        glob_path += "/*";
    }
    let mut paths: Vec<PathBuf> = vec![];
    for path in glob(glob_path.as_str()).unwrap().filter_map(Result::ok) {
        paths.push(path);
    }
    
    if plain {
        plain_print_simple(paths);
    } else {
        let config = get_config!();
        pretty_print_simple(paths, config)
    }
}
