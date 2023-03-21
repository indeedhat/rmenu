use std::vec;

use clap::Parser;

#[derive(Clone)]
pub struct Choice {
    pub value: String,
    pub name: String,
    pub icon: String
}

/// Rust port of Dmenu
#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = None
)]
pub struct CliArgs {
    /// case-insensetive item matching
    #[arg(short, long, default_value="false")]
    insensetive: bool,

    /// scan desktop files for options rather than reading the path
    #[arg(short, long, default_value="false")]
    pub desktop: bool,

    // provide a new line seperated list of options to list
    #[arg(short, long, default_value="")]
    options: String,

    /// string to filter the bins by
    #[arg(default_value="")]
    query: String
}

impl CliArgs {
    /// Creates a choice list from the --options input arg
    pub fn user_options(&self) -> Vec<Choice> {
        let mut choices: Vec<Choice> = vec![];

        if self.options == "" {
            return choices;
        }

        for entry in self.options.split('\n') {
            choices.push(Choice {
                name: entry.to_string(),
                value: entry.to_string(),
                icon: "".to_string()
            })
        }

        choices
    }
}

/// Filter given Choice vec by the --query arg
pub fn filter_entries(args: CliArgs, entries: Vec<Choice>) -> Vec<Choice> {
    let q = if args.insensetive {
        args.query.to_lowercase()
    } else {
        args.query.to_string()
    };

    let mut found: Vec<Choice> = vec![];

    for choice in entries {
        if q == "" 
            || (args.insensetive && choice.name.to_lowercase().contains(&q))
            || (!args.insensetive && choice.name.to_lowercase().contains(&q)
        ) {
            found.push(choice);
        }
    }

    found.sort_by(|a, b| {
        if args.insensetive {
            a.name.to_lowercase().find(&q).cmp(&b.name.to_lowercase().find(&q))
        } else {
            a.name.find(&q).cmp(&b.name.find(&q))
        }
    });

    sort_entries(args, &mut found);
    found
}

pub fn sort_entries(args: CliArgs, entries: &mut Vec<Choice>) {
    let q = if args.insensetive {
        args.query.to_lowercase()
    } else {
        args.query.to_string()
    };

    entries.sort_by(|a, b| {
        if args.insensetive {
            a.name.to_lowercase().find(&q).cmp(&b.name.to_lowercase().find(&q))
        } else {
            a.name.find(&q).cmp(&b.name.find(&q))
        }
    });
}

pub fn alpha_sort_entries(entries: &mut Vec<Choice>) {
    entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
}
