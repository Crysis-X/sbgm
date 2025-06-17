use crate::modules::config::Config;

pub fn next_folder(cfg: &mut Config){
    cfg.set_bg(0);
    cfg.next_folder();
    cfg.save();
}