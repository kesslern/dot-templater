extern crate dot_templater;
extern crate walkdir;

use dot_templater::is_binary;
use dot_templater::is_dir;
use dot_templater::trim_trailing_slash;
use dot_templater::Arguments;
use dot_templater::Config;
use std::env;
use std::error::Error;
use std::fs::File;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Arguments::new(env::args()).unwrap();

    println!("Rules: {}", args.rules);
    println!("From: {}", args.source);
    println!("To: {}", args.dest);

    let file = BufReader::new(File::open(args.rules)?);
    let config = Config::new(file.lines());

    for (key, value) in &config.substitutions {
        println!("Found key: {}", key);
        println!("Found value: {}", value);
    }

    for feature in &config.features {
        println!("Found a feature: {}", feature);
    }

    let copy_from = trim_trailing_slash(&args.source);
    let copy_to = trim_trailing_slash(&args.dest);

    for entry in WalkDir::new(copy_from) {
        let source = entry?;
        let source = source.path();
        let dest = source.to_str().unwrap().replace(copy_from, copy_to);
        let dest = Path::new(&dest);
        println!("Source: {}", source.display());
        println!("Dest: {}", dest.display());
        println!("Dest exists: {}", dest.exists());
        println!("Source is directory: {}", is_dir(source));
        println!("Source is binary: {}", is_binary(&source));
        let source_is_dir = is_dir(source);
        
        if !dest.exists() && source_is_dir {
            fs::create_dir(dest)?;
        } else if !source_is_dir {
            if is_binary(source) {
                fs::copy(source, dest)?;
            } else {
                config.template(source, dest);
            }
        }
        
        println!("-----------------");
    }

    Ok(())
}
