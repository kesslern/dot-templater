extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub struct Config {
    pub features: Vec<String>,
    pub substitutions: HashMap<String, String>,
}

impl Config {
    pub fn new<B: BufRead>(lines: Lines<B>) -> Config {
        let mut config = Config {
            features: Vec::new(),
            substitutions: HashMap::new(),
        };

        for line in lines {
            let line = line.unwrap();

            match Config::parse_line(line) {
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

    fn parse_line(line: String) -> Option<ConfigValue> {
        let mut line = line.trim().to_string();

        if line.is_empty() || line.starts_with("#") {
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

    pub fn template(&self, source: &Path, dest: &Path) {
        println!("Templating {} to {}", source.display(), dest.display());
        let source = BufReader::new(File::open(source).unwrap());
        let mut dest = File::create(dest).unwrap();
        let mut in_disabled_feature = false;

        for line in source.lines() {
            let line = line.unwrap();
            let feature = self.is_feature_enable_or_disable(&line);

            match feature {
                Some(enabled) => {

                    if in_disabled_feature {
                        in_disabled_feature = false;
                    } else if !enabled {
                        in_disabled_feature = true;
                    }
                }
                None => {
                    if !in_disabled_feature {
                        let mut line = line;
                        for (key, value) in &self.substitutions {
                            line = line.replace(key, value);
                        }
                        dest.write_all(line.as_bytes()).expect("oops");
                        dest.write("\n".as_bytes()).expect("oof");
                    }
                }
            }
        }
    }

    fn is_feature_enable_or_disable(&self, line: &str) -> Option<bool> {
        let re = Regex::new("^\\s*### .*$").unwrap();
        if re.is_match(line) {
            println!("Found a feature line: {}", line);
            let found_feature = &line.trim()[3..].trim();
            for feature in &self.features {
                if found_feature == feature {
                    return Some(true);
                }
            }
            return Some(false);
        } else {
            None
        }
    }
}

pub enum ConfigValue {
    Feature(String),
    Substitution { key: String, value: String },
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

pub fn is_dir(file: &Path) -> bool {
    match fs::metadata(file) {
        Ok(metadata) => metadata,
        Err(_) => return false,
    }
    .is_dir()
}

pub fn is_binary(file: &Path) -> bool {
    if is_dir(file) {
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
        if byte == 0b0 {
            return true;
        }
        if iterations > 8000 {
            return false;
        }
        iterations += 1;
    }

    false
}
