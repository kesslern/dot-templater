# These targets don't produce files
.PHONY: install
.PHONY: test

DESTDIR=/usr/bin
EXECFILE=target/release/dot-templater

TEST_RULES=test/rules
TEST_DOTFILES=test/dotfiles
TEST_DEST_DIR=test/dest
TEST_EXPECTED_DIR=test/expected

DIFF_FLAGS=-qNr

SRCS:=$(wildcard src*.rs)

all: build

build: $(SRCS)
	cargo build --release

test:
	rm -rf $(TEST_DEST_DIR)
	mkdir $(TEST_DEST_DIR)
	./$(EXECFILE) $(TEST_RULES) $(TEST_DOTFILES) $(TEST_DEST_DIR); \
	diff $(DIFF_FLAGS) $(TEST_DEST_DIR) $(TEST_EXPECTED_DIR)
	if [ ! -x "$(TEST_DEST_DIR)/binary_file" ]; then \
		@echo "Expected binary_file to be executable."; \
		exit 1; \
	fi
	@echo "*** All tests successfully passed. ***"

install:
	cp $(EXECFILE) $(DESTDIR)
