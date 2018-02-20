# dot-templater
A small, portable program intended for templating dotfiles.

## Features
* Copies files from to another directory while making string substitutions.

### Planned Features
* Enable or disable chunks of dotfiles according to feature flags. 

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
String subsitutions are defined by a key and value separated by the first occurance of `=`. Any line beginning with `#` is ignored. A rules file is defined as
```
#
# String substitions:
#
{SUBSTITION ONE}=123
[font size]=19
asdf=aoeu
```
will replace all occurances of `{SUBSTITION ONE}` with `123`, `[font size]` with `19`, and `asdf` with `aoeu`.

## Development

### Development dependencies
* make
* gcc
* clang-tidy
* clang-format
* valgrind

### Formatting
Code is formatted with `make format`.

### Testing

`make test` copies the files in `test/dotfiles` to `test/dest` according to `test/rules` and compares with `test/expected`. The executable is analyzed with valgrind to prevent memory leaks. Valgrind must report "no leaks possible".

## License

This project is licensed under the [MIT License](LICENSE).
