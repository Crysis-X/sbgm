use crate::modules::config::Config;

pub fn set_folder(cfg: &mut Config, index: usize){
    cfg.set_bg(0);
    cfg.set_folder(index);
    cfg.save();
}