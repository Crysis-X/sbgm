use crate::modules::config::Config;

pub fn index(cfg: &Config){
    println!("{}", cfg.get_bg_index());
}