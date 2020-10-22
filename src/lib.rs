extern crate clap;
extern crate difference;
extern crate regex;

extern crate walkdir;

use difference::Changeset;
use difference::Difference;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::Command;
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

            if let Some(value) = Config::parse_line(line)? {
                match value {
                    ConfigValue::Feature(it) => {
                        config.features.push(it);
                    }
                    ConfigValue::Substitution { key, value } => {
                        config.substitutions.insert(key, value);
                    }
                }
            }
        }

        Ok(config)
    }

    fn run_command(command: String) -> Result<String, Box<dyn Error>> {
        let stdout = Command::new("sh").arg("-c").arg(&command).output()?.stdout;

        Ok(String::from_utf8(stdout)?.trim().to_owned())
    }

    fn parse_line(line: String) -> Result<Option<ConfigValue>, Box<dyn Error>> {
        let mut line = line.trim().to_string();

        if line.is_empty() || line.starts_with('#') {
            return Ok(None);
        }

        match line.chars().position(|c| c == '=') {
            Some(idx) => {
                let mut value = line.split_off(idx);
                value.remove(0);

                if value.starts_with("SHELL ") {
                    let command = value.split_off(6);
                    let result = &Config::run_command(command)?;

                    Ok(Some(ConfigValue::Substitution {
                        key: line,
                        value: result.to_owned(),
                    }))
                } else {
                    Ok(Some(ConfigValue::Substitution { key: line, value }))
                }
            }
            None => Ok(Some(ConfigValue::Feature(line))),
        }
    }

    fn template_file(&self, source: &Path) -> Result<String, Box<dyn Error>> {
        let source = BufReader::new(File::open(source)?);
        let mut feature_stack = Vec::new();
        let mut templated_output = String::new();

        for line in source.lines() {
            let mut line = line?;
            match Config::get_feature(&line) {
                Some(feature) => match feature_stack.last() {
                    Some((last, _)) if *last == feature => {
                        feature_stack.pop();
                    }
                    _ => {
                        feature_stack.push((feature.to_owned(), self.is_enabled(&feature)));
                    }
                },
                None => {
                    if feature_stack.iter().all(|(_, enabled)| *enabled) {
                        for (key, value) in &self.substitutions {
                            line = line.replace(key, value);
                        }
                        templated_output.push_str(&line);
                        templated_output.push('\n');
                    }
                }
            }
        }

        Ok(templated_output)
    }

    fn get_feature(line: &str) -> Option<&str> {
        let re = Regex::new("^\\s*### .*$").expect("fixed regex always valid");
        if re.is_match(line) {
            Some(line.trim()[3..].trim())
        } else {
            None
        }
    }

    fn is_enabled(&self, feature: &str) -> bool {
        self.features.iter().any(|f| f == feature)
    }
}

pub enum Mode {
    Template,
    Diff,
}

pub enum ConfigValue {
    Feature(String),
    Substitution { key: String, value: String },
}

pub struct Arguments<'a> {
    pub rules: &'a str,
    pub source: &'a str,
    pub dest: &'a str,
    pub diff: Mode,
    pub ignore: Vec<&'a str>,
}

impl<'a> Arguments<'a> {
    pub fn new(args: &'a clap::ArgMatches) -> Self {
        let rules = args.value_of("CONFIG").expect("CONFIG is required");
        let mut source = args.value_of("SRC_DIR").expect("SRC_DIR is required");
        let mut dest = args.value_of("DEST_DIR").expect("DEST_DIR is required");
        let diff = if args.is_present("diff") {
            Mode::Diff
        } else {
            Mode::Template
        };

        source = Self::trim_trailing_slash(&source);
        dest = Self::trim_trailing_slash(&dest);
        let ignore = match args.values_of("ignore") {
            Some(value) => value.collect(),
            None => vec![],
        };

        Self {
            rules,
            source,
            dest,
            diff,
            ignore,
        }
    }

    fn trim_trailing_slash(string: &str) -> &str {
        if let Some(b'/') = string.as_bytes().last() {
            &string[..string.len() - 1]
        } else {
            string
        }
    }
}

fn template_files(config: &Config, source: &Path, dest: &Path) -> Result<(), Box<dyn Error>> {
    if is_dir(source) {
        if !dest.exists() {
            fs::create_dir(dest)?;
        }
    } else if is_binary(source) {
        fs::copy(source, dest)?;
    } else {
        let templated = config.template_file(source)?;

        File::create(dest)?.write_all(templated.as_bytes())?;
    }

    Ok(())
}

fn diff_files(config: &Config, source: &Path, dest: &Path) -> Result<(), Box<dyn Error>> {
    if !is_dir(source) {
        if !dest.exists() {
            println!("--- Destination file {} does not exist ---", dest.display());
        } else if is_binary(source) {
            println!(
                "--- Cannot display diff for binary file {} ---",
                source.display()
            );
        } else {
            let existing = fs::read_to_string(dest)?;
            let templated = config.template_file(source)?;
            let changeset = Changeset::new(&existing, &templated, "\n");

            if changeset
                .diffs
                .iter()
                .all(|diff| matches!(*diff, Difference::Same(_)))
            {
                println!(
                    "--- No difference between {} and {} ---",
                    source.display(),
                    dest.display()
                );
            } else {
                println!(
                    "--- Diff results for {} and {} ---",
                    source.display(),
                    dest.display()
                );
                println!("{}", changeset);
            }
        }
    }

    Ok(())
}

pub fn template(
    config: &Config,
    source_dir: &str,
    dest_dir: &str,
    mode: Mode,
    ignore: Vec<&str>,
) -> Result<(), Box<dyn Error>> {
    let ignore_list: Vec<&Path> = ignore.iter().map(|fname| Path::new(fname)).collect();

    let filtered_paths = WalkDir::new(source_dir).into_iter().filter_entry(|entry| {
        !ignore_list.contains(
            &entry
                .path()
                .strip_prefix(&source_dir)
                .expect("entry should always have source_dir prefix"),
        )
    });

    for entry in filtered_paths {
        let source_file = entry?;
        let source_file = source_file.path();
        let dest_file = source_file
            .to_str()
            .expect("expected UTF-8 path")
            .replace(source_dir, dest_dir);
        let dest_file = Path::new(&dest_file);

        match mode {
            Mode::Template => template_files(config, &source_file, &dest_file)?,
            Mode::Diff => diff_files(config, &source_file, &dest_file)?,
        }
    }

    Ok(())
}

fn is_dir(file: &Path) -> bool {
    match fs::metadata(file) {
        Ok(metadata) => metadata,
        Err(_) => return false,
    }
    .is_dir()
}

fn is_binary(file: &Path) -> bool {
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

    for (iteration, byte) in contents.into_iter().enumerate() {
        if byte == 0b0 {
            return true;
        }
        if iteration > 8000 {
            return false;
        }
    }

    false
}
