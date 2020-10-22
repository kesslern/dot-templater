# dot-templater
A small, portable Rust program intended for templating dotfiles across multiple systems.

## Purpose
Storing dotfiles in git repositories allows them to be shared across multiple computers, but this becomes problematic once systems require slightly different configurations. Laptops require battery indicators and WiFi utilities, HiDPI displays use larger fonts... `dot-templater` intends to solve these problems by making it simple to change values or enable/disable chunks of configuration in any file, or content from stdin.

## Features
* Make string substitutions in files/content according to configured key/value pairs.
* Use output from arbitrary shell commands in templated dotfiles (e.g. for passwords with GNU Pass).
* Toggle chunks of files/content per feature flags.
* Copy binary files without templating.
* Preserve file permissions.
* Perform a dry-run to compare expected output against existing files.
* Ignore specific files/folders.

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

```
dot-templater CONFIG
```

Templates content from stdin to stdout according to rules in `CONFIG`.

```
dot-templater --diff CONFIG SRC_DIR DEST_DIR
```

Compares files from `SRC_DIR` modified according to rules in `CONFIG` against the contents of `DEST_DIR`.

### Parameters
* `-i <file> [...files]` or `--ignore <file> [...files]`  
  Excludes a file or directory (and all children) from templating, ie. they will not be copied to the destination directory.
  For example, use `-i .git` for git controlled dotfile repositories.

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

#### Arbitrary Shell Commands
If the `=` separating key/value pairs is immediately proceeded by `SHELL`, dot-templater will run the provided command and use the stdout when templating dotfiles. Providing following line in a config file will substitute any occurrance of `SHELL_COMMAND` with `1234`.
```
SHELL_COMMAND=SHELL echo 1234
```

#### Feature Flags
Any line in the rules configuration file that does not include a `=` character and is not a comment will enable the feature name that matches the line. Dotfiles can designate togglable features with three octothorpes followed by the feature name. Features can be nested.
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

## Releases

[Pre-built Linux binaries are available](https://github.com/kesslern/dot-templater/releases), as well as AUR packages for [releases](https://aur.archlinux.org/packages/dot-templater/) and [building from latest source](https://aur.archlinux.org/packages/dot-templater-git/).

## Release Builds

### Dependencies
* make
* cargo

### Compiling
`make release` compiles a release build, producing `target/release/dot-templater`.

### Testing
`make test` copies the files in `test/dotfiles` to `test/dest` according to `test/rules` and compares with `test/expected`.

## License
[MIT](LICENSE)
