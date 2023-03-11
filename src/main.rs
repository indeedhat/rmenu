use clap::Parser;

mod rmenu;
mod fs;

fn main() -> Result<(), ()> {
    let args = rmenu::CliArgs::parse();
    let options = if args.user_options().len() > 0 {
        args.user_options()
    } else if args.desktop{
        fs::list_bins_from_desktop_files().unwrap()
    } else {
        fs::list_bins_from_path().unwrap()
    };

    println!("{}", options.len());
    for option in rmenu::filter_entries(args, options) {
        println!("{} -> {}", option.name, option.value);
    }

    Ok(())
}

