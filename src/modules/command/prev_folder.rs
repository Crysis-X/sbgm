use crate::modules::config::Config;

pub fn prev_folder(cfg: &mut Config){
    cfg.set_bg(0);
    cfg.prev_folder();
    cfg.save();
}