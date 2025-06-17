use crate::modules::config::Config;

pub fn next(cfg: &mut Config){
    cfg.next_bg();
    cfg.save();
}