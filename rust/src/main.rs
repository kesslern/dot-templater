extern crate dot_templater;
extern crate walkdir;

use dot_templater::get_config;
use dot_templater::trim_trailing_slash;
use std::error::Error;
use std::fs::File;
use std::fs;
use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::process;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Wrong arguments: expected 4 found {}", args.len()-1);
        process::exit(1);
    }

    let rules_file = &args[1];
    let copy_from = &args[2];
    let copy_to = &args[3];

    println!("Rules: {}", rules_file);
    println!("From: {}", copy_from);
    println!("To: {}", copy_to);
    
    let file = BufReader::new(File::open("mew")?);
    let config = get_config(file.lines());
 
    for (key, value) in config.substitutions {
        println!("Found key: {}", key);
        println!("Found value: {}", value);
    }

    for feature in config.features {
        println!("Found a feature: {}", feature);
    }

    let copy_from = trim_trailing_slash(copy_from);
    let copy_to = trim_trailing_slash(copy_to);
    
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
