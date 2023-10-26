mod directory;
mod git;
mod table;
mod terminal;

use std::process;
use std::{collections, env};

fn iterate_over_repos(path: &str) -> Vec<collections::HashMap<String, String>> {
    let mut rows: Vec<collections::HashMap<String, String>> = vec![];

    let folders: Vec<String> = directory::get_folders(path);
    for folder in folders {
        let status: String = git::status(folder.as_str());
        if status == git::NO_GIT_REPO {
            let mut subrows: Vec<collections::HashMap<String, String>> =
                iterate_over_repos(folder.as_str());
            rows.append(&mut subrows);
        } else {
            let size: u64 = directory::get_directory_size(folder.as_str());
            let files: Vec<String> = directory::get_all_files(folder.as_str());
            let mut row: collections::HashMap<String, String> =
                collections::HashMap::<String, String>::new();
            row.insert(String::from("Path"), folder);
            row.insert(String::from("Status"), status);
            row.insert(String::from("Size"), directory::size_to_hr(size));
            row.insert(String::from("Files"), files.len().to_string());
            row.insert(String::from("Bytes"), size.to_string());
            rows.push(row);
        }
    }
    rows
}

fn help() {
    println!("Usage of git-manager [action] <...options>");
    println!("\tactions:");
    println!("\t\thelp    : show current message");
    println!("\t\tversion : show version of the program");
    println!("\t\tlookup  : search for repos on specified");
    println!("\t\t\t<path> : route the the repos");
}

fn version() {
    println!("git-manager 1.0.14");
}

fn error(message: &str) {
    println!("{}", message);
    help();
    process::exit(0x0100);
}

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() < 2 {
        error("missing arguments");
    }
    let action: &String = &arguments[1];
    if action == "version" {
        if arguments.len() >= 3 {
            error(format!("too few arguments for command {}", action).as_str());
        }
        version();
    } else if action == "help" {
        if arguments.len() >= 3 {
            error(format!("too much arguments for command {}", action).as_str());
        }
        help();
    } else if action == "lookup" {
        if arguments.len() <= 2 {
            error(format!("too few arguments for command {}", action).as_str());
        }
        if arguments.len() >= 4 {
            error(format!("too much arguments for command {}", action).as_str());
        }
        let path: &String = &arguments[2];
        let mut repos: Vec<collections::HashMap<String, String>> =
            iterate_over_repos(path.as_str());

        repos.sort_by(
            |a: &collections::HashMap<String, String>, b: &collections::HashMap<String, String>| {
                b["Bytes"]
                    .parse::<i64>()
                    .unwrap()
                    .cmp(&a["Bytes"].parse::<i64>().unwrap())
            },
        );

        let headers: Vec<String> = vec![
            String::from("Path"),
            String::from("Status"),
            String::from("Size"),
            String::from("Files"),
        ];
        table::print_with_headers(&repos, &headers);
    } else {
        error(format!("invalid action \"{}\"", action).as_str());
    }
}
