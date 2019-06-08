extern crate dot_templater;
extern crate walkdir;

use dot_templater::Arguments;
use dot_templater::Config;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Arguments::new(env::args()).unwrap();

    println!("Rules: {}", args.rules);
    println!("From: {}", args.source);
    println!("To: {}", args.dest);

    let file = BufReader::new(File::open(args.rules)?);
    let config = Config::new(file.lines())?;

    for (key, value) in &config.substitutions {
        println!("Found key: {}", key);
        println!("Found value: {}", value);
    }

    for feature in &config.features {
        println!("Found a feature: {}", feature);
    }

    config.template(&args.source, &args.dest).expect("Err");

    Ok(())
}
