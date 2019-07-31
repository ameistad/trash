use std::{env, fs};
use std::process::Command;

extern crate clap;
use clap::{App, Arg};

extern crate chrono;
use chrono::Local;

mod utils;

fn main() {
    let matches = App::new("trash")
        .author("Andreas Meistad <andreas@meidev.co>")
        .about("Moves files and folders in the Trash")
        .version("v0.1.0")
        .arg(
            Arg::with_name("items_to_trash")
            .required(true)
            .multiple(true)
            .help("Specify files and folders to put in trash.")
        )
        .get_matches();

    let items_to_trash: Vec<String> = matches
        .values_of("items_to_trash")
        .unwrap()
        .map(|s| s.to_string()) // nødvendig?
        .collect();

    let deduped_items_to_trash = utils::dedup_vec(&items_to_trash);
    let trash_items = TrashItems::new(deduped_items_to_trash);
    trash_items.trash();
}

#[derive(Debug)]
struct TrashItems {
    trash_directory: String,
    trash_items: Vec<String>,
    trash_items_duplicates: Vec<(String, String)>,
    trash_items_not_found: Vec<String>
}

impl TrashItems {

    fn new(items_to_trash: Vec<String>) -> TrashItems {
        let trash_directory = match env::var_os("HOME") {
            Some(dir) => format!("{}/.Trash", dir.to_str().unwrap().to_string()),
            None => panic!("Failed to get home directory. Make sure the environment variable $HOME is set.")
        };

        let trash_directory_items: Vec<String> = fs::read_dir(&trash_directory)
            .expect("Failed to get trash directory.")
            .map(|item| item.unwrap().file_name().into_string().unwrap())
            .collect();

        let current_directory_items: Vec<String> = fs::read_dir(env::current_dir().unwrap())
            .expect("Failed to get current directory.")
            .map(|item| item.unwrap().file_name().into_string().unwrap())
            .collect();


        let mut trash_items = Vec::new();
        let mut trash_items_duplicates = Vec::new();
        let mut trash_items_not_found = Vec::new();

        for item in items_to_trash {
            if !current_directory_items.contains(&item) {
                trash_items_not_found.push(item);
            } else if trash_directory_items.contains(&item) {
                let timestamp = Local::now().format("%H-%M-%S-%6f").to_string();
                let new_name_item = format!("{}/{} {}", &trash_directory, item, timestamp);
                trash_items_duplicates.push((item, new_name_item));
            } else {
                trash_items.push(item);
            }
        };

        TrashItems {
            trash_directory,
            trash_items,
            trash_items_duplicates,
            trash_items_not_found
        }
    }

    fn trash (&self) {
        if self.trash_items.len() > 0 {
            let mut editor_process = Command::new("mv")
                .args(&self.trash_items)
                .arg(&self.trash_directory)
                .spawn()
                .expect("Failed to move files to Trash");

            editor_process.wait().expect("Failed to run move command");
        }

        if self.trash_items_duplicates.len() > 0 {
            for item in &self.trash_items_duplicates {
                let mut editor_process = Command::new("mv")
                    .arg(&item.0)
                    .arg(&item.1)
                    .spawn()
                    .expect("Failed to move files to Trash");

                editor_process.wait().expect("Failed to run move command");
            }
        }

        self.message();
    }

    fn message (&self) {
        println!("Moved {} items to {}", self.trash_items.len() + self.trash_items_duplicates.len(), self.trash_directory);
    }
}
