extern crate regex;

#[derive(Debug, Clone)]
pub struct Pattern {
    pub regex: Regex,
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.regex.as_str().eq(other.regex.as_str())
    }
}

impl Eq for Pattern {}

impl Hash for Pattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.regex.as_str().hash(state);
    }
}

impl Pattern {
    pub fn new(pattern: &str) -> Self {
        let regex = Regex::new(pattern).unwrap();
        Pattern { regex }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PrettySelector {
    Extension(String),
    MultipleExtensions(Vec<String>),
    Pattern(Pattern),
    ReadOnly,
    EmptyDir,
    File,
    Dir,
    Symlink,
}

#[derive(Debug, Hash, Clone)]
pub enum PrettyPos {
    Foreground,
    Background,
}

#[derive(Clone, Debug)]
pub enum PrettyColor {
    Simple(String),
    True(u8, u8, u8),
}

#[derive(Clone, Debug)]
pub enum PrettyStyle {
    Bold,
    Italic,
    Underline,
    Dimmed,
    Normal,
    DimmedBold,
}

pub type PrettyModifiers = (PrettyPos, PrettyColor, PrettyStyle);
pub type PrettyIcon = String;
pub type PrettyEntry = (PrettyModifiers, PrettyIcon);
pub type PrettyConfig = IndexMap<PrettySelector, PrettyEntry>;

use std::{error::Error, hash::Hash};

use indexmap::{indexmap, IndexMap};
use regex::Regex;

macro_rules! entry {
    ($pos:expr, $color:expr, $style:expr, $icon:expr) => {
        (($pos, $color, $style), String::from($icon))
    };
}

macro_rules! fg_bold {
    ($color:expr, $icon:expr) => {
        entry!(PrettyPos::Foreground, $color, PrettyStyle::Bold, $icon)
    };
}

macro_rules! fg_norm {
    ($color:expr, $icon:expr) => {
        entry!(PrettyPos::Foreground, $color, PrettyStyle::Normal, $icon)
    };
}

macro_rules! pattern {
    ($pattern:expr) => {
        PrettySelector::Pattern(Pattern::new($pattern))
    };
}

macro_rules! multiple {
    ($exts_vec:expr) => {
        PrettySelector::MultipleExtensions($exts_vec)
    };
}

//TODO: Fix this so that macros work.

pub fn standard_config() -> PrettyConfig {
    //Build the standard configuration on the fly...
    //Probably faster than file I/O honestly, but uses more memory.
    indexmap! {
        pattern!(r"package(?:-lock)?.json\b") => fg_bold!(PrettyColor::Simple("yellow".to_string()), ""),
        pattern!(r#"\.git(?:ignore|modules)?\b"#) => fg_norm!(PrettyColor::True(245, 76, 39), ""),
        multiple!(vec![
            "xml".to_string(),
            "html".to_string(),
            "xhtml".to_string(),
            "svg".to_string()
        ]) => fg_bold!(PrettyColor::True(255, 165, 0), ""),
        PrettySelector::Extension("md".to_string()) => fg_bold!(PrettyColor::Simple("bright green".to_string()), ""),
        multiple!(vec![
            "png".to_string(),
            "jpg".to_string(),
            "jpeg".to_string(),
            "jxr".to_string(),
            "exr".to_string(),
        ]) => fg_bold!(PrettyColor::Simple("bright green".to_string()), ""),
        PrettySelector::Extension("db".to_string()) => fg_bold!(PrettyColor::Simple("bright green".to_string()), ""),
        PrettySelector::Extension("exe".to_string()) => fg_bold!(PrettyColor::Simple("bright green".to_string()), ""),
        PrettySelector::Extension("dll".to_string()) => fg_bold!(PrettyColor::Simple("bright green".to_string()), ""),
        PrettySelector::MultipleExtensions(vec![
            "mp4".to_string(),
            "webm".to_string(),
            "gif".to_string(),
            "osp".to_string(),
        ]) => fg_bold!(PrettyColor::Simple("bright green".to_string()), ""),
        PrettySelector::MultipleExtensions(vec![
            "zip".to_string(),
            "7z".to_string(),
            "gz".to_string(),
            "tar".to_string()
        ]) => fg_bold!(PrettyColor::Simple("purple".to_string()), ""),
        PrettySelector::Extension("py".to_string()) => fg_bold!(PrettyColor::Simple("yellow".to_string()), ""),
        PrettySelector::Extension("svelte".to_string()) => fg_bold!(PrettyColor::True(255, 62, 0), " "),
        PrettySelector::Extension("rs".to_string()) => fg_bold!(PrettyColor::True(226, 114, 91), ""),
        PrettySelector::Extension("toml".to_string()) => fg_bold!(PrettyColor::True(226, 114, 91), ""),
        PrettySelector::MultipleExtensions(vec![
            "ini".to_string(),
            "config".to_string(),
            "conf".to_string()
        ]) => fg_bold!(PrettyColor::Simple("green".to_string()), ""),
        PrettySelector::MultipleExtensions(vec![
            "sln".to_string(),
            "csproj".to_string(),
            "fsproj".to_string()
        ]) => fg_bold!(PrettyColor::Simple("green".to_string()), ""),
        PrettySelector::Extension("cs".to_string()) => fg_bold!(PrettyColor::Simple("bright cyan".to_string()), ""),
        PrettySelector::Extension("js".to_string()) => fg_bold!(PrettyColor::Simple("yellow".to_string()), ""),
        PrettySelector::Extension("ts".to_string()) => fg_bold!(PrettyColor::Simple("cyan".to_string()), ""),
        PrettySelector::Extension("json".to_string()) => fg_bold!(PrettyColor::Simple("purple".to_string()), ""),
        PrettySelector::MultipleExtensions(vec!["java".to_string(), "jar".to_string()]) => fg_bold!(PrettyColor::Simple("bright red".to_string()), ""),
        PrettySelector::Extension("fs".to_string()) => fg_bold!(PrettyColor::Simple("magenta".to_string()), ""),
        PrettySelector::Extension("txt".to_string()) => fg_bold!(PrettyColor::Simple("green".to_string()), ""),
        PrettySelector::Symlink => fg_bold!(PrettyColor::Simple("white".to_string()), ""),
        PrettySelector::ReadOnly => ((
            PrettyPos::Background,
            PrettyColor::Simple("green".to_string()),
            PrettyStyle::Bold,
        ), String::from("")),
        PrettySelector::EmptyDir => fg_bold!(PrettyColor::Simple("blue".to_string()), ""),
        PrettySelector::Dir =>
        fg_bold!(PrettyColor::Simple("blue".to_string()), ""),
        PrettySelector::File => fg_bold!(PrettyColor::Simple("green".to_string()), "")
    }
}

macro_rules! get_config {
    () => {
        config::standard_config()
        // if let Ok(path) = std::env::var("PRETTY_LS_CONFIG_PATH") {
        //     let file = std::fs::File::open(path);
        //     if file.is_err() {
        //         config::standard_config()
        //     } else {
        //         let reader = std::io::BufReader::new(file.unwrap());
        //         let config = serde_json::from_reader::<
        //             std::io::BufReader<std::fs::File>,
        //             config::PrettyConfig,
        //         >(reader);
        //         if config.is_err() {
        //             config::standard_config()
        //         } else {
        //             config.unwrap()
        //         }
        //     }
        // } else {
        //     config::standard_config()
        // }
    };
}
