use std::process::{Command, Stdio};
pub fn draw(
    path: String, 
    display: String, 
    mpv_args: String, 
    mpvpaper_args: String
    ){
    let _ = Command::new("pkill").args(["-x", "mpvpaper"]).status();
    let mut cmd = Command::new("mpvpaper");
    cmd.stdout(Stdio::inherit())
    .stderr(Stdio::inherit());
    
    cmd.args(["-o", &mpv_args]);
    let mpvpaper: Vec<&str> = mpvpaper_args.split(' ').collect();
    for a in mpvpaper {
        if !a.trim().is_empty() {
            cmd.arg(a);
        }
    }

    cmd.args([display, path]);
    let _ = cmd.status();
}