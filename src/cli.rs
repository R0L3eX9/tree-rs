use std::env;

pub struct Cli {
    args: Vec<String>
}

impl Cli {
    pub fn new() -> Cli {
        return Cli {
            args: env::args().collect(),
        };
    }

    pub fn get_flags(self) -> Vec<(String, String)> {
        let mut flags: Vec<(String, String)> = Vec::new();
        let args = self.args;
        if args.len() == 2 && args[1] == "--help" {
            flags.push((args[1].clone(), String::from("0")));
        } else {
            for (idx, arg) in args.iter().enumerate() {
                if idx > 1 && arg.contains("--") == false && args[idx - 1].contains("--") == true {
                    flags.push((args[idx - 1].clone(), arg.clone()));
                }
            }
        }
        return flags;
    }
}

pub fn cli_help() {
    println!("Usage:");
    println!("  tree-rs --FLAG <value>");
    println!("Flags:");
    println!("  --L - specifies the depth level to explore");
    println!("  --D - specifies the directory to explore, default is '.'");
    println!("Example: ");
    println!("  tree-rs --L 3 --D './src'");
}
