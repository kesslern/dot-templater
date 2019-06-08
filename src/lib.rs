extern crate regex;
extern crate walkdir;

use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

pub struct Config {
    pub features: Vec<String>,
    pub substitutions: HashMap<String, String>,
}

impl Config {
    pub fn new<B: BufRead>(lines: Lines<B>) -> Result<Config, Box<dyn Error>> {
        let mut config = Config {
            features: Vec::new(),
            substitutions: HashMap::new(),
        };

        for line in lines {
            let line = line?;

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

        Ok(config)
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

    fn template_file(&self, source: &Path, dest: &Path) -> Result<(), Box<dyn Error>> {
        println!("Templating {} to {}", source.display(), dest.display());
        let source = BufReader::new(File::open(source)?);
        let mut dest = File::create(dest)?;
        let mut in_disabled_feature = false;

        for line in source.lines() {
            let line = line?;
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
                        dest.write_all(line.as_bytes())?;
                        dest.write("\n".as_bytes())?;
                    }
                }
            }
        }

        Ok(())
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

    pub fn template(&self, source_dir: &str, dest_dir: &str) -> Result<(), Box<dyn Error>> {
        for entry in WalkDir::new(source_dir) {
            let source_file = entry?;
            let source_file = source_file.path();
            let dest_file = source_file.to_str().unwrap().replace(source_dir, dest_dir);
            let dest_file = Path::new(&dest_file);
            println!("Source: {}", source_file.display());
            println!("Dest: {}", dest_file.display());
            println!("Dest exists: {}", dest_file.exists());
            println!("Source is directory: {}", is_dir(source_file));
            println!("Source is binary: {}", is_binary(&source_file));
            let source_file_is_dir = is_dir(source_file);

            if !dest_file.exists() && source_file_is_dir {
                fs::create_dir(dest_file)?;
            } else if !source_file_is_dir {
                if is_binary(source_file) {
                    fs::copy(source_file, dest_file)?;
                } else {
                    self.template_file(source_file, dest_file)?;
                }
            }

            println!("-----------------");
        }
        
        Ok(())
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

        let mut source: String = match args.next() {
            Some(mut arg) => arg,
            None => return Err("No source directory provided."),
        };

        let mut dest: String = match args.next() {
            Some(mut arg) => arg,
            None => return Err("No destination directory provided."),
        };

        Arguments::trim_trailing_slash(&mut source);
        Arguments::trim_trailing_slash(&mut dest);

        Ok(Arguments {
            rules,
            source,
            dest,
        })
    }

    fn trim_trailing_slash(string: &mut String) {
        let len = string.len();
        let has_trailing_slash = match string.as_bytes().last() {
            Some(byte) => *byte == '/' as u8,
            None => return,
        };

        if has_trailing_slash {
            string.truncate(len - 1);
        }
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
