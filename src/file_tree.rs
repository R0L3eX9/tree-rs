use std::fs;
use std::io::Result;
use std::path::PathBuf;

pub struct Tree {
    directories: usize,
    files: usize,
}

pub fn summery(tree: Tree) {
    println!("\n{} directories, {} files", tree.directories, tree.files);
}

fn walk(root: PathBuf, max_depth: usize, depth: usize, prefix: String) -> Result<(usize, usize)> {
    let (mut dirs, mut files) = (0, 0);
    let mut paths: Vec<PathBuf> = fs::read_dir(root)?.map(|entry| entry.unwrap().path()).collect();
    paths.sort();
    for (idx, path) in paths.iter().enumerate() {
        let name = path.file_name().unwrap().to_str().unwrap();

        if name.starts_with('.') == true {
            continue;
        }
        let mut tree_name = if idx == paths.len() - 1 {
            format!("{}└── ", prefix)
        } else {
            format!("{}├── ", prefix)
        };

        tree_name = if path.is_dir() == true {
            format!("{}\x1b[94m{}\x1b[0m", tree_name, name)
        } else {
            format!("{}{}", tree_name, name)
        };
        println!("{}", tree_name);
        if path.is_dir() {
            dirs += 1;

            let new_pref = if idx == paths.len() - 1 {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            if depth + 1 < max_depth {
                let res = walk(path.to_owned(), max_depth, depth + 1, new_pref)?;
                dirs += res.0;
                files += res.1;
            }
        } else {
            files += 1;
        }
    }
    Ok((dirs, files))
}

pub fn explore(root: PathBuf, max_depth: usize) -> Result<Tree> {
    match walk(root, max_depth, 0, format!("")) {
        Ok(tree) => Ok(Tree {
            directories: tree.0,
            files: tree.1,
        }),
        Err(err) => Err(err),
    }
}
