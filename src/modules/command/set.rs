use crate::modules::config::Config;

pub fn set(cfg: &mut Config, index: usize){
    cfg.set_bg(index);
    cfg.save();
}