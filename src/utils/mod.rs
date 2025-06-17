use std::fs;
use crate::{consts, modules::config::Config, utils};
use serde::{Serialize};
use serde_json::{self, value::Serializer};
use std::env;

pub fn cfg_dir() -> String {
    format!("{}/.config/sbgm", home_dir())
}
pub fn cfg_path() -> String {
    format!("{}/config.json", cfg_dir())
}
pub fn home_dir() -> String {
    env::var("HOME").expect("Please set $HOME")
}

pub fn check_index<T>(vec: &Vec<T>, x: usize) -> bool {
        if vec.len() <= x {
            false
        } else {
            true
        }
    }

pub fn create_default_config() -> () {
    if let Err(e) = fs::create_dir(cfg_dir()) {
        match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => panic!("can't create sbgm folder: {:?}", e)
        };
    }
    let default = Config::new(
        0,
        0, 
        vec![String::from("~/bg/")], 
        None, 
        None,
        None
    ).serialize(Serializer).expect("code error");
    let res = fs::write(cfg_path(), default.to_string());
    res.unwrap_or_else(|e| {
        match e.kind() {
            std::io::ErrorKind::AlreadyExists => {
                fs::remove_file(cfg_path()).expect("can't remove a cfg file");
                create_default_config();
            },
            _ => {}
        }
    })
}

pub fn get_media_files(path: &str) -> Vec<String> {
    let mut media_files = Vec::new(); 
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        let ext = ext.to_string_lossy().to_lowercase();
                        if consts::MEDIA_EXT.contains(&ext.as_str()) {
                            if let Some(path_str) = path.to_str() {
                                media_files.push(path_str.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    media_files
}
pub fn replace_tilde(s: &str) -> String {
    s.replacen("~", &utils::home_dir(), 1)
}