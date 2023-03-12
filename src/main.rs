use clap::Parser;

use gtk::prelude::*;

use crate::rmenu::alpha_sort_entries;

mod gui;
mod rmenu;
mod fs;
mod list_entry;

fn main() -> Result<(), ()> {

    let args = rmenu::CliArgs::parse();
    let mut options = if args.user_options().len() > 0 {
        print!("user opts");
        args.user_options()
    } else if args.desktop{
        print!("desktop");
        fs::list_bins_from_desktop_files().unwrap()
    } else {
        println!("bins");
        fs::list_bins_from_path().unwrap()
    };

    alpha_sort_entries(&mut options);

    let app = gui::new(options);
    app.run();


    Ok(())
}
