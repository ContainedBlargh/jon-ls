// extern crate dirs;
// extern crate serde;
// extern crate serde_json;

use indexmap::IndexMap;
// use serde::{Deserialize, Serialize};
// use serde::{Deserializer, Serializer};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, Hash, Clone /*Deserialize*/)]
pub enum PrettySelector {
    ReadOnly,
    Extension(String),
    MultipleExtensions(Vec<String>),
    EmptyDir,
    File,
    Dir,
    Symlink,
}

// impl Serialize for PrettySelector {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         match self {
//             ReadOnly => serializer.serialize_str("ReadOnly"),
//             Extension(ext) => serializer.serialize_str(format!("Extension({})", ext).as_str()),
//             EmptyDir => serializer.serialize_str("EmptyDir"),
//             File => serializer.serialize_str("File"),
//             Dir => serializer.serialize_str("Dir"),
//             Symlink => serializer.serialize_str("Symlink"),
//         }
//     }
// }

#[derive(Debug, PartialEq, Eq, Hash, Clone /*Serialize, Deserialize*/)]
pub enum PrettyPos {
    Foreground,
    Background,
}

#[derive(Clone, /*Deserialize,*/ Debug)]
pub enum PrettyColor {
    Simple(String),
    True(u8, u8, u8),
}

// impl Serialize for PrettyColor {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match self {
//             Simple(color) => serializer.serialize_str(color),
//             True(r, g, b) => serializer.collect_seq(vec![r, g, b].iter()),
//         }
//     }
// }

#[derive(Clone, /*Deserialize,*/ Debug)]
pub enum PrettyStyle {
    Bold,
    Italic,
    Underline,
    Dimmed,
    Normal,
    DimmedBold,
}

// impl Serialize for PrettyStyle {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match self {
//             PrettyStyle::Bold => serializer.serialize_str("Bold"),
//             PrettyStyle::Italic => serializer.serialize_str("Italic"),
//             PrettyStyle::Underline => serializer.serialize_str("Underline"),
//             PrettyStyle::Dimmed => serializer.serialize_str("Dimmed"),
//             PrettyStyle::Normal => serializer.serialize_str("Normal"),
//             PrettyStyle::DimmedBold => serializer.serialize_str("DimmedBold")
//         }
//     }
// }

pub type PrettyModifiers = (PrettyPos, PrettyColor, PrettyStyle);
pub type PrettyIcon = String;
pub type PrettyEntry = (PrettyModifiers, PrettyIcon);
pub type PrettyConfig = IndexMap<PrettySelector, PrettyEntry>;

// pub fn get_config_path() -> PathBuf {
//     let path = std::env::var("PRETTY_LS_CONFIG_PATH")
//         .map(|it| PathBuf::from(it))
//         .unwrap_or(dirs::config_dir().unwrap().join(PathBuf::from(".pls.json")));
//     path
// }

use indexmap::indexmap;

pub fn standard_config() -> PrettyConfig {
    //Build the standard configuration on the fly...
    //Probably faster than file I/O honestly, but uses more memory.
    indexmap! {
        PrettySelector::MultipleExtensions(vec![
            "xml".to_string(),
            "html".to_string(),
            "xhtml".to_string(),
            "svg".to_string()
        ]) => ((
            PrettyPos::Foreground,
            PrettyColor::True(255, 165, 0),
            PrettyStyle::Bold
        ), String::from("")),
        PrettySelector::Extension("md".to_string()) => ((
            PrettyPos::Foreground,
            PrettyColor::Simple("green".to_string()),
            PrettyStyle::Bold
        ), String::from("")),
        PrettySelector::MultipleExtensions(vec![
            "png".to_string(),
            "jpg".to_string(),
            "jpeg".to_string(),
            "jxr".to_string(),
            "exr".to_string(),
        ]) => ((
            PrettyPos::Foreground,
            PrettyColor::Simple("bright green".to_string()),
            PrettyStyle::Bold
        ), String::from("")),
        PrettySelector::MultipleExtensions(vec![
            "mp4".to_string(),
            "webm".to_string(),
            "gif".to_string(),
            "osp".to_string(),
        ]) => ((
            PrettyPos::Foreground,
            PrettyColor::Simple("bright green".to_string()),
            PrettyStyle::Bold
        ), String::from("")),
        PrettySelector::MultipleExtensions(vec![
            "zip".to_string(),
            "7z".to_string(),
            "gz".to_string(),
            "tar".to_string()
        ]) =>
        ((
            PrettyPos::Foreground,
            PrettyColor::Simple("purple".to_string()),
            PrettyStyle::Bold,
        ), String::from("")),
        PrettySelector::Extension("py".to_string()) =>
        ((
            PrettyPos::Foreground,
            PrettyColor::Simple("yellow".to_string()),
            PrettyStyle::Bold,
        ), String::from("")),
        PrettySelector::Extension("svelte".to_string()) =>
        ((
            PrettyPos::Foreground,
            PrettyColor::True(255, 62, 0),
            PrettyStyle::Bold,
        ), String::from(" ")),
        PrettySelector::Extension("rs".to_string()) =>
        ((
            PrettyPos::Foreground,
            PrettyColor::True(226, 114, 91),
            PrettyStyle::Bold,
        ), String::from("")),
        PrettySelector::Extension("toml".to_string()) =>
        ((
            PrettyPos::Foreground,
            PrettyColor::True(226, 114, 91),
            PrettyStyle::Bold,
        ), String::from("")),
        PrettySelector::MultipleExtensions(vec![
            "ini".to_string(),
            "config".to_string(),
            "conf".to_string()
        ]) =>
        ((
            PrettyPos::Foreground,
            PrettyColor::Simple("green".to_string()),
            PrettyStyle::Bold,
        ), String::from("")),
        PrettySelector::MultipleExtensions(vec![
            "sln".to_string(),
            "csproj".to_string(),
            "fsproj".to_string()
        ]) =>
        ((
            PrettyPos::Foreground,
            PrettyColor::Simple("green".to_string()),
            PrettyStyle::Bold,
        ), String::from("")),
        PrettySelector::Extension("cs".to_string()) =>
        ((
            PrettyPos::Foreground,
            PrettyColor::Simple("bright cyan".to_string()),
            PrettyStyle::Bold,
        ), String::from("")),
        PrettySelector::Extension("js".to_string()) =>
        ((
            PrettyPos::Foreground,
            PrettyColor::Simple("yellow".to_string()),
            PrettyStyle::Bold,
        ), String::from("")),
        PrettySelector::Extension("json".to_string()) =>
        ((
            PrettyPos::Foreground,
            PrettyColor::Simple("purple".to_string()),
            PrettyStyle::Bold,
        ), String::from("")),
        PrettySelector::Extension("java".to_string()) =>
        ((
            PrettyPos::Foreground,
            PrettyColor::Simple("bright red".to_string()),
            PrettyStyle::Bold,
        ), String::from("")),
        PrettySelector::Extension("fs".to_string()) =>
        ((
            PrettyPos::Foreground,
            PrettyColor::Simple("magenta".to_string()),
            PrettyStyle::Bold,
        ), String::from("")),
        PrettySelector::Extension("txt".to_string()) =>
        ((
            PrettyPos::Foreground,
            PrettyColor::Simple("green".to_string()),
            PrettyStyle::Bold,
        ), String::from("")),
        PrettySelector::Symlink => ((
            PrettyPos::Foreground,
            PrettyColor::Simple("white".to_string()),
            PrettyStyle::Bold,
        ), String::from("")),
        PrettySelector::ReadOnly => ((
            PrettyPos::Background,
            PrettyColor::Simple("green".to_string()),
            PrettyStyle::Bold,
        ), String::from("")),
        PrettySelector::EmptyDir => ((
            PrettyPos::Foreground,
            PrettyColor::Simple("blue".to_string()),
            PrettyStyle::DimmedBold,
        ), String::from("")),
        PrettySelector::Dir =>
        ((
            PrettyPos::Foreground,
            PrettyColor::Simple("blue".to_string()),
            PrettyStyle::Bold,
        ), String::from("")),
        PrettySelector::File =>
        ((
            PrettyPos::Foreground,
            PrettyColor::Simple("green".to_string()),
            PrettyStyle::Bold,
        ), String::from("")),
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
