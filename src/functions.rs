use colored::*;
use rayon::prelude::*;
use std::io::{self, Write};
use std::path::{self, Path};
use std::path::PathBuf;
use terminal_size::terminal_size;

use regex::Regex;
use std::cmp::min;
use std::fs::read_dir;

#[inline]
fn get_term_width() -> usize {
    let re = Regex::new(r"Width\((\d+)\)").unwrap();
    let width = terminal_size().map(|t| t.0).unwrap_or_else(|| {
        eprintln!("Could not determine terminal size...");
        terminal_size::Width(200)
    });
    let wfmt: String = format!("{:?}", width);
    let cap = re.captures(wfmt.as_str()).unwrap().get(1).unwrap().as_str();
    cap.parse::<usize>().unwrap()
}

use crate::config::*;

trait PrettyMatches {
    fn matches(self, path: &Path, meta: &std::fs::Metadata) -> Option<PrettySelector>;
}

use crate::config::PrettyColor::*;
use crate::config::PrettyPos::*;
use crate::config::PrettySelector::*;

#[inline]
fn path_extension_matches(path: &Path, extension: &str) -> bool {
    path.extension()
        .map(|it| it.to_str().unwrap_or("").eq_ignore_ascii_case(extension))
        .unwrap_or(false)
}

#[inline]
fn path_matches_regex(path: &Path, pattern: &crate::config::Pattern) -> bool {
    path.file_name()
        .map(|it| pattern.regex.is_match(it.to_str().unwrap_or("")))
        .unwrap_or(false)
}

impl PrettyMatches for PrettySelector {
    fn matches(self, path: &Path, meta: &std::fs::Metadata) -> Option<PrettySelector> {
        return match self {
            Pattern(pattern) if path_matches_regex(path, &pattern) => Some(Pattern(pattern)),
            MultipleExtensions(exts) if exts.iter().any(|ext| path_extension_matches(path, ext)) => Some(MultipleExtensions(exts)),
            Extension(ext) if path_extension_matches(path, &ext) => Some(Extension(ext)),
            Symlink if meta.file_type().is_symlink() => Some(self),
            ReadOnly if meta.permissions().readonly() => Some(self),
            EmptyDir
                if read_dir(path)
                    .map(|mut i| i.next().is_none())
                    .unwrap_or(false) =>
            {
                Some(self)
            }
            Dir if meta.is_dir() => Some(self),
            File if meta.is_file() => Some(self),
            _ => None,
        };
    }
}

fn apply_modifiers_and_add_icon(
    path: &Path,
    modifiers: &PrettyModifiers,
    icon: &String,
) -> ColoredString {
    let (pos, color, style) = modifiers;
    let path = path
        .file_name()
        .map(|it| it.to_str().unwrap())
        .unwrap_or("");
    let mut acc = format!("{} {}", icon, path).normal();
    acc = match (pos, color) {
        (Foreground, Simple(color)) => acc.color(color.as_str()),
        (Background, Simple(color)) => acc.on_color(color.as_str()),
        (Foreground, True(r, g, b)) => acc.truecolor(*r, *g, *b),
        (Background, True(r, g, b)) => acc.on_truecolor(*r, *g, *b),
        _ => acc,
    };
    match style {
        PrettyStyle::Bold => acc.bold(),
        PrettyStyle::Italic => acc.italic(),
        PrettyStyle::Underline => acc.underline(),
        PrettyStyle::Dimmed => acc.dimmed(),
        PrettyStyle::Normal => acc.normal(),
        PrettyStyle::DimmedBold => acc.bold().dimmed(),
        _ => acc,
    }
}

use crate::config::PrettyConfig;

fn prettify_path(path: &Path, config: &PrettyConfig) -> Option<ColoredString> {
    for (selector, entry) in config {
        if let Ok(meta) = path.metadata() {
            if let Some(_selector) = selector.clone().matches(path, &meta) {
                let (modifiers, icon) = entry;
                return Some(apply_modifiers_and_add_icon(path, modifiers, icon));
            }
        }
    }
    None
}

fn plain_path(path: &Path) -> Option<String> {
    let is_empty = read_dir(path)
        .map(|mut i| i.next().is_none())
        .unwrap_or(false);
    let name_opt = path.file_name().map(|it| it.to_str().unwrap());
    if let Some(name) = name_opt {
        return if let Ok(meta) = path.metadata() {
            if meta.is_dir() && is_empty {
                Some(format!("{}/-", name))
            } else if meta.is_dir() {
                Some(format!("{}/", name))
            } else {
                Some(name.to_string())
            }
        } else {
            None
        };
    }
    None
}

macro_rules! output_simple {
    ($paths:expr) => {
        let mut stdout = io::stdout().lock();
        let n = $paths.len();
        for i in 0..n {
            stdout
                .write_fmt(format_args!(" {} \n", &$paths[i]))
                .unwrap();
            stdout.flush().unwrap();
        }
    };
}

macro_rules! output_grid {
    ($paths:expr) => {
        let mut stdout = io::stdout().lock();
        let n = $paths.len();
        let longest = $paths
            .clone()
            .into_par_iter()
            .max_by_key(|s| s.len())
            .unwrap()
            .len();
        let term_width = get_term_width();
        let per_line = term_width / (longest + 3);
        let mut i = 0;
        while i < n {
            for j in 0..min(per_line, n - i) {
                if j + i > n {
                    break;
                }
                let mut trailing_space = String::from("");
                for _ in 0..(longest - $paths[i + j].len()) {
                    trailing_space += " ";
                }
                stdout
                    .write_fmt(format_args!(" {} {}", &$paths[i + j], trailing_space))
                    .unwrap();
            }
            stdout.write_all(b"\n").unwrap();
            stdout.flush().unwrap();
            i += per_line;
        }
    };
}

pub fn pretty_print(paths: Vec<PathBuf>, config: PrettyConfig, lines: bool) {
    let pretty_paths: Vec<ColoredString> = paths
        .into_par_iter()
        .map(|path| prettify_path(&path, &config))
        .filter(|opt| opt.is_some())
        .map(|opt| opt.unwrap())
        .collect();
    if lines {
        output_simple!(pretty_paths);
    } else if !pretty_paths.is_empty() {
        output_grid!(pretty_paths);
    }
}

pub fn plain_print(paths: Vec<PathBuf>, lines: bool) {
    let plain_paths: Vec<String> = paths
        .into_iter()
        .filter_map(|path| plain_path(&path))
        .collect();
    if lines {
        output_simple!(plain_paths);
    } else if !plain_paths.is_empty() {
        output_grid!(plain_paths);
    }
}
