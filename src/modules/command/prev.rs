use crate::modules::config::Config;

pub fn prev(cfg: &mut Config){
    cfg.prev_bg();
    cfg.save();
}