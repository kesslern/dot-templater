extern crate dot_templater;

use dot_templater::parse_line;
use dot_templater::ConfigValue;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let mut variables = HashMap::new();
    let mut features: Vec<String> = Vec::new();
    let file = BufReader::new(File::open("mew")?);

    for line in file.lines() {
        let line = line.unwrap();

        match parse_line(line) {
            Some(value) => match value {
                ConfigValue::Feature(it) => {
                    features.push(it);
                }
                ConfigValue::Substitution { key, value } => {
                    variables.insert(key, value);
                }
            },
            None => (),
        }
    }

    for (key, value) in variables {
        println!("Found key: {}", key);
        println!("Found value: {}", value);
    }

    for feature in features {
        println!("Found a feature: {}", feature);
    }

    Ok(())
}
