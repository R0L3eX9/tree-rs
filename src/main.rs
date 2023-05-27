mod tree;

use std::path::PathBuf;
use crate::tree::file_tree::{explore, summery};

fn main() {
    let level: usize = 3;
    let root = String::from(".");
    println!("\x1b[94m{}\x1b[0m", root);
    let tree = explore(PathBuf::from(root), level);
    summery(tree);
}
