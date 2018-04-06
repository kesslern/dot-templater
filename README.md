# dot-templater
A small, portable program intended for templating dotfiles across multiple systems.

## Purpose
Many developers store their dotfiles in git repositories, allowing them to share their dotfiles across multiple computers. When one system requires a slight change in configuration, a branch is created for those differences. Any further changes intended for the main branch must be merged without bringing in branch-specific changes . This becomes more complicated when multiple features are involved. Different configuration may be required to show battery indicators on portable systems, use larger fonts on HiDPI systems, show Wifi indicators where necessary, or change other values depending on operating system. dot-templater intends to solve these problems.

## Features
* Make string substitutions in files according to configuration files.
* Include or exclude chunks of files according to configuration file feature flags.
* Binary files are copied without templating.
* File permissions are preserved -- executable scripts will remain executable.

## Known Bugs
I fixed all I could find. Please report any bugs, difficulties, or suggestions in the issue tracker.

### Planned Features
You tell me!

## Building

### Build dependencies
* make
* gcc

Build the project with `make build`, producing executable `dot-templater`. 

## Usage
```
dot-templater RULES SRC_DIR DEST_DIR
```

Copies files from `SRC_DIR` to `DEST_DIR` according to rules in `RULES`.

### Rules Format
Any line beginning with `#` is ignored. Rules file can contain key/value substitutions and feature flags.

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

## Development
### Dependencies
* make
* gcc
* clang-tidy
* clang-format
* valgrind

### Compiling
`make` compiles the code and produces a `dot-templater` executable.

### Formatting
Code is formatted with `make format`.

### Testing
`make test` copies the files in `test/dotfiles` to `test/dest` according to `test/rules` and compares with `test/expected`. The executable is analyzed with valgrind to prevent memory leaks. Valgrind must report "no leaks possible".

## License
[MIT](LICENSE)
