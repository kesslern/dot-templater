extern crate dot_templater;
extern crate walkdir;

use dot_templater::get_config;
use dot_templater::trim_trailing_slash;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use walkdir::WalkDir;

struct Arguments {
    rules: String,
    source: String,
    dest: String,
}

fn main() -> Result<(), Box<dyn Error>> {

    let args = Arguments::new(env::args()).unwrap();

    println!("Rules: {}", args.rules);
    println!("From: {}", args.source);
    println!("To: {}", args.dest);

    let file = BufReader::new(File::open("mew")?);
    let config = get_config(file.lines());

    for (key, value) in config.substitutions {
        println!("Found key: {}", key);
        println!("Found value: {}", value);
    }

    for feature in config.features {
        println!("Found a feature: {}", feature);
    }

    let copy_from = trim_trailing_slash(&args.source);
    let copy_to = trim_trailing_slash(&args.dest);

    for entry in WalkDir::new(copy_from) {
        let entry = entry?;
        let path = entry.path();
        let dest = path.to_str().unwrap().replace(copy_from, copy_to);
        println!("Source: {}", path.display());
        println!("Dest: {}", dest);
        let attr = fs::metadata(path)?;
        println!("Is directory: {}", attr.is_dir());
        println!("-----------------");
    }

    Ok(())
}

impl Arguments {
    pub fn new(mut args: env::Args) -> Result<Arguments, &'static str> {
        args.next();

        let rules = match args.next() {
            Some(arg) => arg,
            None => return Err("No rules file provided."),
        };

        let source = match args.next() {
            Some(arg) => arg,
            None => return Err("No source directory provided."),
        };

        let dest = match args.next() {
            Some(arg) => arg,
            None => return Err("No destination directory provided."),
        };

        Ok(Arguments {
            rules,
            source,
            dest,
        })
    }
}
