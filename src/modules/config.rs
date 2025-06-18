use serde::{Serialize, Deserialize};
use serde_json::{self, value::Serializer};
use crate::{modules::command, utils::{self, check_index}};
use std::{self, fs, io};
use crate::modules::error::ParserError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    bg_index: usize,
    folder_index: usize,
    folders: Vec<String>,
    display: String,
    mpv_args: String,
    mpvpaper_args: String
}
impl Config {
    pub fn new(
        bg_index: usize,
        mut folder_index: usize,
        folders: Vec<String>,
        display: Option<String>,
        mpv_args: Option<String>,
        mpvpaper_args: Option<String>
    ) -> Config {
        if !folders.is_empty() {
          if !check_index(&folders, folder_index) {
            folder_index = folders.len() - 1;
          }  
        } 
        Config {
            bg_index: bg_index,
            folder_index: folder_index,
            folders: Config::fix_folders(folders), 
            display: display.unwrap_or_else(|| String::from("DP-1")),
            mpv_args: mpv_args.unwrap_or_else(|| String::new()),
            mpvpaper_args: mpvpaper_args.unwrap_or_else(|| String::new()) 
        }
    } 
    pub fn draw(&mut self, path: Option<String>){
        let bg_path = path.unwrap_or_else(|| self.get_bg());
        command::draw::draw(
            bg_path,
            self.get_display().to_string(),
            self.get_mpv_args().to_string(),
            self.get_mpvpaper_args().to_string()
        );
    }
    pub fn get_display(&self) -> &str {
        &self.display
    }
    pub fn get_mpv_args(&self) -> &str {
        &self.mpv_args
    }
    pub fn get_mpvpaper_args(&self) -> &str {
        &self.mpvpaper_args
    }
    pub fn get_bg(&mut self) -> String {
        let images = self.get_bgs();
        let i = self.bg_index;
        if !check_index(&images, i) {
            self.bg_index = images.len() - 1;
            return self.get_bg();
        }
        images[i].clone()
    }
    pub fn get_bg_index(&self) -> usize {
        self.bg_index
    }
    pub fn get_folder_index(&self) -> usize {
        self.folder_index
    }
    pub fn get_bgs(&self) -> Vec<String> {
        let images = utils::get_media_files(&self.folders[self.folder_index]);
        if images.is_empty() {
            panic!("can't load bg files");        
        };
        images
    }
    pub fn set_bg(&mut self, i: usize){
        self.bg_index = i;
    }
    pub fn set_folder(&mut self, i: usize){
        self.folder_index = i;
    }
    pub fn next_bg(&mut self) -> usize {
        let images = self.get_bgs();
;        let i = self.bg_index + 1;
        if utils::check_index(&images, i) {
            self.bg_index = i;
        } else {
            self.bg_index = 0;
        }
        self.bg_index
    }
    pub fn next_folder(&mut self) -> usize {
        let i = self.folder_index + 1;
        if utils::check_index(&self.folders, i) {
            self.folder_index = i;
        } else {
            self.folder_index = 0;
        }
        self.folder_index
    }
    pub fn prev_bg(&mut self) -> usize {
        let images = self.get_bgs();
        let i = self.bg_index - 1;
        if utils::check_index(&images, i) {
            self.bg_index = i;
        } else {
            self.bg_index = images.len() - 1;
        }
        self.bg_index
    }
    pub fn prev_folder(&mut self) -> usize{
        let i = self.folder_index - 1;
        if utils::check_index(&self.folders, i) {
            self.folder_index = i;
        } else {
            self.folder_index = self.folders.len() - 1;
        }
        self.folder_index
    }
    pub fn save(&self){
        let result = self.serialize(Serializer).expect("can't save config, it's broken");
    match fs::write(utils::cfg_path(), result.to_string()){
        Err(e) => {
            match e.kind() {
                io::ErrorKind::AlreadyExists => {
                    let _ = fs::remove_file(utils::cfg_path());
                    self.save()
                },
                _ => {
                    println!("config path: {}", utils::cfg_path());
                    panic!("can't save config");
                }
            }
        },
        _ => {}
    }
}
    pub fn parse() -> Result<Config, ParserError> {
        match fs::read_to_string(utils::cfg_path()) {
            Ok(result) => { 
                let val: Result<Config, ParserError> = serde_json::from_str(&result).map_err(ParserError::JSON);
                let config: Result<Config, ParserError> = match val {
                    Ok(cfg) => {
                        Ok(Config::new(
                        cfg.bg_index, 
                        cfg.folder_index, 
                        Config::fix_folders(cfg.folders),
                        Some(cfg.display),
                        Some(cfg.mpv_args),
                        Some(cfg.mpvpaper_args)
                        ))
                    },
                    Err(e) => Err(e)
                };
                config
            },
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::NotFound => {
                            utils::create_default_config();
                            println!("created default config file on: {}", utils::cfg_path());
                            println!("configure it)");
                    },
                    _ => {
                        println!("config path: {}", utils::cfg_path());
                        panic!("can't read config file")
                    }
                }
                println!("can't read cfg file: {}", utils::cfg_path());
                Err(ParserError::from(e)) 
            }
        }
    }
    fn fix_folders(folders: Vec<String>) -> Vec<String> {
        let mut fixed_folders: Vec<String> = vec![];
        for folder in folders {
            fixed_folders.push(utils::replace_tilde(&folder));
        }
        fixed_folders
    }
}

