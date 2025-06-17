use crate::modules::config::Config;

pub fn folder_index(cfg: &Config){
    println!("{}", cfg.get_folder_index());
}