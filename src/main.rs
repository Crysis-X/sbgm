mod modules;
mod utils;
mod consts;

use clap::Parser;
use modules::cli::{Cli, CliCommands};
use modules::command;

use crate::modules::config::Config;

fn main() {
        let cli = Cli::parse();
        let mut cfg = Config::parse()
        .expect(&format!("can't load cfg file at: {}", utils::cfg_path()));
        match &cli.command {
                Some(CliCommands::FolderIndex) => command::folder_index::folder_index(&cfg),
                Some(CliCommands::Index) => command::index::index(&cfg),
                Some(CliCommands::Next) => command::next::next(&mut cfg),
                Some(CliCommands::NextFolder) => command::next_folder::next_folder(&mut cfg),
                Some(CliCommands::Prev) => command::prev::prev(&mut cfg),
                Some(CliCommands::PrevFolder) => command::prev_folder::prev_folder(&mut cfg),
                Some(CliCommands::Set { index }) => command::set::set(&mut cfg, index.clone()),
                Some(CliCommands::SetFolder { index}) => command::set_folder::set_folder(&mut cfg, index.clone()),
                _ => {}
        };
        match cli.command {
                None => &cfg.draw(cli.path.clone()),
                Some(CliCommands::FolderIndex) | Some(CliCommands::Index) => &(),
                _ => &cfg.draw(None)
        };
}
