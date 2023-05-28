mod file_tree;
mod cli;

use std::path::PathBuf;
use crate::file_tree::{explore, summery};
use crate::cli::{Cli, cli_help};

fn main() {
    let mut level: usize = 3;
    let mut root = String::from(".");
    let cli = Cli::new();
    for flag in cli.get_flags() {
        match flag.0.as_str() {
            "--L" => level = flag.1.parse().unwrap(),
            "--D" => root = flag.1,
            _ => { 
                cli_help();
                return;
            },
        }
    }

    println!("\x1b[94m{}\x1b[0m", root);
    match explore(PathBuf::from(root.clone()), level) {
        Ok(tree) => {
            summery(tree);
        },
        Err(_) => {
            println!("Could not explore the specified path");
            cli_help();
        }
    };
}
