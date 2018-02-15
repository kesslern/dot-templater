all: format dottemplater test

dottemplater: config.o util.o dottemplater.o
	gcc util.o config.o dottemplater.o -o dot-templater

config.o:
	gcc -std=c11 -O2 -Wall -Wextra -Wpedantic -Werror -c config.c

util.o:
	gcc -std=c11 -O2 -Wall -Wextra -Wpedantic -Werror -c util.c

dottemplater.o:
	gcc -std=c11 -O2 -Wall -Wextra -Wpedantic -Werror -c dottemplater.c


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

