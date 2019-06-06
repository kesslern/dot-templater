use std::collections::HashMap;
use std::io::BufRead;
use std::io::Lines;

pub struct Config {
    pub features: Vec<String>,
    pub substitutions: HashMap<String, String>
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

pub fn trim_trailing_slash(string: &str) -> &str {
    let last_byte = string.as_bytes().last().unwrap();
    if *last_byte == '/' as u8 {
        &string[..string.len()-1]
    } else {
        string
    }
}
