use std::fs;
use std::io::Result;
use std::path::PathBuf;

fn explore_dir(root: PathBuf, depth: usize) -> Result<(usize, usize)> {
    let root = fs::read_dir(root)?;
    let (mut dirs, mut files) = (0, 0);
    for entry in root {
        let path = entry?;
        let entry_type = path.metadata()?;
        let file_name = match path.file_name().into_string() {
            Ok(res) => res,
            Err(_) => continue,
        };
        // ├ ─ └ │
        if entry_type.is_dir() == true {
            println!("└\x1b[92m{file_name:>width$}\x1b[0m", file_name = file_name, width = depth + file_name.len());
            explore_dir(path.path(), depth + 4)?;
            dirs += 1;
        } else {
            files += 1;
            println!("{file_name:>width$}", file_name = file_name, width = depth + file_name.len());
        }
    }
    return Ok((dirs, files));
}

fn main() {
    match explore_dir(PathBuf::from("./"), 0) {
        Ok(data) => println!("{} directories, {} files", data.0, data.1),
        Err(_) => println!("Couldn't explore the file tree!"),
    };
}
