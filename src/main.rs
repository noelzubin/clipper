#[macro_use]
extern crate clap;

extern crate clipboard;

mod action;
mod config;
mod error;
mod notes;
mod trello;
mod utils;

use action::Action;
use notes::Notes;
use std::process;
use trello::Trello;

use clap::App;
use clipboard::x11_clipboard::{Primary, X11ClipboardContext};
use clipboard::ClipboardProvider;
use config::Config;

fn get_selection() -> String {
    let mut ctx: X11ClipboardContext<Primary> = ClipboardProvider::new().unwrap();
    ctx.get_contents().unwrap()
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();

    let conf = Config::new();

    let (subcommand, _args) = matches.subcommand();

    let action: Box<dyn Action> = match subcommand {
        "bookmark" => Box::new(Trello::new(conf)),
        "notes" => Box::new(Notes::new(conf)),
        _ => {
            eprintln!("please provide a valid subcommand");
            process::exit(0);
        }
    };

    let selection = get_selection();
    action.perform(&selection);
}
