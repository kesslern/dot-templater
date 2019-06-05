use std::collections::HashMap;
use std::io::BufRead;
use std::io::Lines;

pub struct Config {
    features: Vec<String>,
    substitutions: HashMap<String, String>
}

pub enum ConfigValue {
    Feature(String),
    Substitution { key: String, value: String },
}

pub fn parse_line(line: String) -> Option<ConfigValue> {
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

pub fn get_config(lines: Lines) -> Config {
    let mut variables = HashMap::new();
    let mut features: Vec<String> = Vec::new();

    for line in lines {
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

}
