# These targets are not file names
.PHONY: install test all build release

DESTDIR=/usr/bin
EXECFILE=target/release/dot-templater

TEST_RULES=test/rules
TEST_DOTFILES=test/dotfiles
TEST_DEST_DIR=test/dest
TEST_EXPECTED_DIR=test/expected
TEST_IGNORE_ARG=--ignore ignored_file

DIFF_FLAGS=-qNr

SRCS:=$(wildcard src*.rs)

all: build

build: $(SRCS)
	cargo build

release: $(SRCS)
	cargo build --release --locked --all-features

$(EXECFILE): release

test: $(EXECFILE)
	rm -rf $(TEST_DEST_DIR)
	mkdir $(TEST_DEST_DIR)
	./$(EXECFILE) $(TEST_RULES) $(TEST_DOTFILES) $(TEST_DEST_DIR) $(TEST_IGNORE_ARG); \
	cat $(TEST_DOTFILES)/template | ./$(EXECFILE) $(TEST_RULES) > $(TEST_DEST_DIR)/stdout; \
	diff $(DIFF_FLAGS) $(TEST_DEST_DIR) $(TEST_EXPECTED_DIR)
	if [ ! -x "$(TEST_DEST_DIR)/binary_file" ]; then \
		@echo "Expected binary_file to be executable."; \
		exit 1; \
	fi
	@echo "*** All tests successfully passed. ***"

install:
	cp $(EXECFILE) $(DESTDIR)
