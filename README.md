# dot-templater
A small, portable Rust program intended for templating dotfiles across multiple systems.

## Purpose
Storing dotfiles in git repositories allows them to be shared across multiple computers, but this becomes problematic once systems require slightly different configurations. Laptops require battery indicators and WiFi utilities, HiDPI displays use larger fonts... `dot-templater` intends to solve these problems by making it simple to change values or enable/disable chunks of configuration in any file.

## Features
* Make string substitutions in files according to configured key/value pairs.
* Toggle chunks of files per feature flags.
* Binary files are copied without templating.
* File permissions are preserved.

## Planned Features
Feature requests are welcome!

## Building

### Build dependencies
* cargo

Build the project with `cargo build`. Run with `cargo run`.

## Usage
```
dot-templater CONFIG SRC_DIR DEST_DIR
```

Copies files from `SRC_DIR` to `DEST_DIR` according to rules in `CONFIG`.

### Config Format
Any line beginning with `#` is ignored. Config file can contain key/value substitutions and feature flags.

#### Key/Value Substitutions
String subsitutions are defined by a key and value separated by the first occurance of `=`. The following configuration file:
```
{SUBSTITION ONE}=123
[font size]=19
asdf=aoeu
```
will replace all occurances of `{SUBSTITION ONE}` with `123`, `[font size]` with `19`, and `asdf` with `aoeu`.

#### Feature Flags
Any line in the rules configuration file that does not include a `=` character and is not a comment will enable the feature name that matches the line. Dotfiles can designate togglable features with three octothorpes followed by the feature name. 
```
FEATURE1
```
Will enable `FEATURE1` in any dotfiles:
```
This line will always be included
### FEATURE1
This line will only be included when FEATURE1 is enabled.
### FEATURE1
This line will always be included.
```

## Release Builds

### Dependencies
* make
* cargo

### Compiling
`make` compiles a release build, producing `target/release/dot-templater`.

### Testing
`make test` copies the files in `test/dotfiles` to `test/dest` according to `test/rules` and compares with `test/expected`.

## License
[MIT](LICENSE)
