extern crate clap;
extern crate dot_templater;
extern crate walkdir;

use clap::App;
use clap::Arg;
use dot_templater::Arguments;
use dot_templater::Config;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::{self};
use std::process;

fn main() {
    let matches = App::new("dot-templater")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Nathan Kessler <nathant93@gmail.com>")
        .arg(
            Arg::with_name("CONFIG")
                .help("Path to rules file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("SRC_DIR")
                .help("Path to directory containing template files")
                .index(2),
        )
        .arg(
            Arg::with_name("DEST_DIR")
                .help("Path to generate output files in")
                .index(3),
        )
        .arg(
            Arg::with_name("diff")
                .short("d")
                .long("diff")
                .help("Diff mode"),
        )
        .arg(
            Arg::with_name("ignore")
                .short("i")
                .long("ignore")
                .help("Ignore file")
                .multiple(true)
                .min_values(0)
                .takes_value(true),
        )
        .after_help(concat!(
            "Copy files from SRC_DIR to DEST_DIR using rules defined in CONFIG.\n\n",
            "Rules configuration:\n",
            "  Keys and values are separated by the first occurance of '=' in a line.\n",
            "  In each copied file, each key is replaced by the associated value.\n\n",
            "Feature configuration:\n",
            "  Any line in the rules file without an '=' character is an enabled feature.\n",
            "  Features can be defined in dotfiles with three octothorpes followed by any\n",
            "  whitespace and the feature name. Features that are not enabled in the rules\n",
            "  will be excluded in the templated file.\n",
        ))
        .get_matches();

    let args = Arguments::new(&matches);

    let config = File::open(args.rules).unwrap_or_else(|err| {
        eprintln!("Error while opening config file: {}", err);
        process::exit(1);
    });

    let config = BufReader::new(config);
    let config = Config::new(config.lines()).unwrap_or_else(|err| {
        eprintln!("Error while parsing config file: {}", err);
        process::exit(1);
    });

    if args.source.is_some() && args.dest.is_some() {
        let source = &args.source.unwrap();
        let dest = &args.dest.unwrap();

        dot_templater::template(&config, &source, &dest, args.diff, args.ignore)
    } else {
        dot_templater::template_lines(&config, io::stdin().lock().lines())
    }
    .unwrap_or_else(|err| {
        eprintln!("Error while performing templating: {}", err);
        process::exit(1);
    });
}
