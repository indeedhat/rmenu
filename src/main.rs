use clap::Parser;

use gtk::prelude::*;
use modes::{dmenu::Dmenu, text_select::TextSelect, MenuMode};

mod gui;
mod rmenu;
mod list_entry;
mod modes;

fn main() -> Result<(), ()> {
    let args = rmenu::CliArgs::parse();
    let mode: Box<dyn MenuMode> = if args.user_options().len() > 0 {
        Box::new(TextSelect::new(args))
    // } else if args.desktop{
    //     print!("desktop");
    //     fs::list_bins_from_desktop_files().unwrap()
    } else {
        Box::new(Dmenu::new(args))
    };

    let app = gui::new(mode);
    app.run();


    Ok(())
}
