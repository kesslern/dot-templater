use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::Lines;
use std::io::Read;

pub struct Config {
    pub features: Vec<String>,
    pub substitutions: HashMap<String, String>,
}

pub enum ConfigValue {
    Feature(String),
    Substitution { key: String, value: String },
}

fn parse_line(line: String) -> Option<ConfigValue> {
    let mut line = line.trim().to_string();

    if line.is_empty() {
        return None;
    }

    match line.chars().position(|c| c == '=') {
        Some(idx) => {
            let mut var = line.split_off(idx);
            var.remove(0);
            return Some(ConfigValue::Substitution {
                key: line,
                value: var,
            });
        }
        None => {
            println!("Found a feature: {}", line);
            return Some(ConfigValue::Feature(line));
        }
    }
}

pub fn get_config<B: BufRead>(lines: Lines<B>) -> Config {
    let mut config = Config {
        features: Vec::new(),
        substitutions: HashMap::new(),
    };

    for line in lines {
        let line = line.unwrap();

        match parse_line(line) {
            Some(value) => match value {
                ConfigValue::Feature(it) => {
                    config.features.push(it);
                }
                ConfigValue::Substitution { key, value } => {
                    config.substitutions.insert(key, value);
                }
            },
            None => (),
        }
    }
    config
}

pub struct Arguments {
    pub rules: String,
    pub source: String,
    pub dest: String,
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

pub fn trim_trailing_slash(string: &str) -> &str {
    let last_byte = string.as_bytes().last().unwrap();
    if *last_byte == '/' as u8 {
        &string[..string.len() - 1]
    } else {
        string
    }
}

pub fn is_binary(file: &str) -> bool {
    let metadata = match fs::metadata(file) {
        Ok(metadata) => metadata,
        Err(_) => return false,
    };

    if metadata.is_dir() {
        return false;
    }

    let mut file = match File::open(file) {
        Ok(file) => file,
        Err(_) => return false,
    };

    let mut contents: Vec<u8> = Vec::new();
    match file.read_to_end(&mut contents) {
        Ok(_) => (),
        Err(_) => return false,
    };

    let mut iterations = 0;

    for byte in contents {
        if byte == b'0' {
            return true;
        }
        if iterations > 8000 {
            return false;
        }
        iterations += 1;
    }

    false
}
