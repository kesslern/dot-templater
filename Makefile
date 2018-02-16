.PHONY: test

EXECFILE=dot-templater
CFLAGS=-std=c11 -O2 -Wall -Wextra -Wpedantic -Werror
SRCS:=$(wildcard *.c)
OBJS:=$(SRCS:.c=.o )

CLANG_FORMAT_OPTS="{\
		BasedOnStyle: llvm,\
		IndentWidth: 4,\
		AllowShortFunctionsOnASingleLine: None,\
		KeepEmptyLinesAtTheStartOfBlocks: false,\
		IndentCaseLabels: false,\
		BreakBeforeBraces: Linux}"

CLANG_TIDY_OPTS=\
	readability-braces-around-statements,misc-macro-parentheses

all: format build

format:
	clang-format \
		-style=$(CLANG_FORMAT_OPTS)\
		-i $(SRCS)

	clang-tidy \
		-fix \
		-fix-errors \
		-header-filter=.* \
		--checks=$(CLANG_TIDY_OPTS) \
		*.c \
		-- -I .

%.o : %.c
	gcc $(CFLAGS) -c $< -o $@

build: $(OBJS)
	gcc $(OBJS) -o $(EXECFILE)

test:
	valgrind --track-origins=yes --leak-check=full --show-leak-kinds=all --error-exitcode=1 ./$(EXECFILE) test/rules test/dotfiles test/dest
	diff -qNr test/dest test/expected

clean:
	rm -f $(OBJS)
