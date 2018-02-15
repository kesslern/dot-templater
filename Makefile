all: format build test

build:
	gcc -std=c11 -O2 -Wall -Wextra -Wpedantic -Werror util.c config.c dottemplater.c -o dot-templater


format:
	clang-format -style="{BasedOnStyle: llvm, IndentWidth: 4, AllowShortFunctionsOnASingleLine: None, KeepEmptyLinesAtTheStartOfBlocks: false, IndentCaseLabels: false, BreakBeforeBraces: Linux}" -i *.c

	clang-tidy \
	-fix \
	-fix-errors \
	-header-filter=.* \
	--checks=readability-braces-around-statements,misc-macro-parentheses \
	*.c \
	-- -I.

test:
	valgrind --track-origins=yes ./dot-templater rules dotfiles dest

