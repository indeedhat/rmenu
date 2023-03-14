use std::env::{self, VarError};
use std::error::Error;
use std::{fs, vec};
use faccess::PathExt;
use freedesktop_entry_parser::parse_entry;

#[derive(Clone)]
pub struct Choice {
    pub value: String,
    pub name: String,
    pub icon: String
}

/// list all the binaries found it the directories described in the users PATH
pub fn list_bins_from_path() -> Result<Vec<Choice>, VarError> {
    let mut bins: Vec<String> = vec![];
    // let mut bins: HashMap<String, String> = HashMap::new();
    let path = env::var("PATH")?;

    for dir in path.split(":") {
        for bin in bins_in_dir(dir.to_string()) {
            if bins.contains(&bin.to_string()) {
                continue;
            }

            // this seems excessive
            bins.push(bin.clone());
        }
    }

    let mut choices: Vec<Choice> = vec![];
    for bin in bins {
        choices.push(Choice {
            value: bin.to_string(),
            name: bin.to_string(),
            icon: "".to_string()
        })
    }

    Ok(choices)
}

/// list binaries in the given directory
fn bins_in_dir(path: String) -> Vec<String> {
    let mut bins: Vec<String> = vec![];
    let paths = match fs::read_dir(path) {
        Ok(paths) => paths,
        Err(_) => return bins
    };

    for path in paths {
        match path {
            Ok(path) => {
                if path.path().executable() {
                    bins.push(path.file_name().to_str().unwrap().to_string());
                }
            },
            _ => continue
        }
    }

    bins
}

/// Search for and parse .desktop files to create our Choice list
pub fn list_bins_from_desktop_files() -> Result<Vec<Choice>, Box<dyn Error>> {
    let dirs = vec![
        "/usr/share/applications",
        "/usr/local/share/applications",
        "~/.local/applications",
    ];
    let mut choices: Vec<Choice> = vec![];

    for dir in dirs {
        for desktop_file in desktop_files_in_dir(dir.to_string()) {
            match parse_desktop_file(desktop_file) {
                Ok(entry) => {
                    choices.push(entry);
                },
                Err(_) => println!("failed to find dir {}", dir)
            };
        }
    }

    Ok(choices)
}

/// Scann the given directory for .desktop files and return the found paths as a vector
fn desktop_files_in_dir(path: String) -> Vec<String> {
    let mut desktop_files: Vec<String> = vec![];
    let paths = match fs::read_dir(path) {
        Ok(paths) => paths,
        Err(_) => return desktop_files
    };

    for path in paths {
        match path {
            Ok(path) => {
                if path.path().to_str().unwrap_or("").ends_with(".desktop") {
                    desktop_files.push(path.path().to_str().unwrap().to_string());
                }
            },
            Err(_) => println!("failed to parse file")
        }
    }

    desktop_files
}


/// parse a .desktop file into a Choice
fn parse_desktop_file(path: String) -> Result<Choice, Box<dyn Error>> {
    let entry = parse_entry(path)?;
    let desktop_entry = entry.section("Desktop Entry");

    Ok(Choice {
        name: desktop_entry.attr("Name").unwrap_or("").to_string(),
        value: desktop_entry.attr("Exec").unwrap_or("").to_string(),
        icon: desktop_entry.attr("Icon").unwrap_or("").to_string()
    })
}
