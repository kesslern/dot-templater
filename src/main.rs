extern crate dot_templater;
extern crate walkdir;

use dot_templater::Arguments;
use dot_templater::Config;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process;

fn main() {
    let args = Arguments::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error: could not parse arguments: {}", err);
        println!();
        print_help();
        process::exit(1);
    });

    let config = File::open(args.rules).unwrap_or_else(|err| {
        eprintln!("Error while opening config file: {}", err);
        process::exit(1);
    });

    let config = BufReader::new(config);
    let config = Config::new(config.lines()).unwrap_or_else(|err| {
        eprintln!("Error while parsing config file: {}", err);
        process::exit(1);
    });

    config
        .template(&args.source, &args.dest)
        .unwrap_or_else(|err| {
            eprintln!("Error while performing templating: {}", err);
            process::exit(1);
        });
}

fn print_help() {
    let name = env::args().next().unwrap();
    println!("usage: {} CONFIG SRC_DIR DEST_DIR", name);
    println!();
    println!("Copy files from SRC_DIR to DEST_DIR using rules defined in CONFIG.");
    println!();
    println!("Rules configuration:");
    println!("  Keys and values are separated by the first occurance of '=' in a line.");
    println!("  In each copied file, each key is replaced by the associated value.");
    println!();
    println!("Feature configuration:");
    println!("  Any line in the rules file without an '=' character is an enabled feature.");
    println!("  Features can be defined in dotfiles with three octothorpes followed by any");
    println!("  whitespace and the feature name. Features that are not enabled in the rules");
    println!("  will be excluded in the templated file.");
}
