extern crate dot_templater;

use dot_templater::get_config;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let file = BufReader::new(File::open("mew")?);
    let config = get_config(file.lines());
    
    for (key, value) in config.substitutions {
        println!("Found key: {}", key);
        println!("Found value: {}", value);
    }

    for feature in config.features {
        println!("Found a feature: {}", feature);
    }

    Ok(())
}
