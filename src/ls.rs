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
use rayon::prelude::ParallelBridge;
use std::os::windows::prelude::*;
use std::path::PathBuf;

fn main() {
    let mut glob_path = String::from("*");
    let mut plain = false;
    let mut copyright = false;
    let mut single_lines = false;
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
        ap.refer(&mut single_lines).add_option(
            &["-l", "--lines"],
            StoreTrue,
            "List items on separate lines.",
        );
        ap.parse_args_or_exit();
    }
    if copyright {
        println!("(c) Jon Voigt 2022 - This software can be used at your own risk, but for your own gain as well.");
    }
    let sep = std::path::MAIN_SEPARATOR;
    if glob_path.ends_with(sep) {
        glob_path = format!("{}{}*", glob_path, sep);
    }
    let mut paths: Vec<PathBuf> = glob(glob_path.as_str())
        .unwrap()
        .filter_map(Result::ok)
        .collect();
    if paths.is_empty() {
        paths = glob(format!("{}{}*", glob_path, sep).as_str())
            .unwrap()
            .filter_map(Result::ok)
            .filter(|it| {
                if cfg!(windows) {
                    it.metadata()
                        .map(|m| m.file_attributes() & 0x2 > 0)
                        .unwrap_or(false)
                        && it
                            .file_name()
                            .map(|it| it.to_str().map(|it| !it.starts_with("NTUSER.DAT")))
                            .flatten()
                            .unwrap_or(false)
                } else {
                    true
                }
            })
            .collect();
    }

    if plain {
        plain_print(paths, single_lines);
    } else {
        let config = get_config!();
        pretty_print(paths, config, single_lines);
    }
}
