#!/usr/bin/env bash

clang-format -style="{BasedOnStyle: llvm, IndentWidth: 4, AllowShortFunctionsOnASingleLine: None, KeepEmptyLinesAtTheStartOfBlocks: false, IndentCaseLabels: false, BreakBeforeBraces: Linux}" -i *.c

clang-tidy \
  -fix \
  -fix-errors \
  -header-filter=.* \
  --checks=readability-braces-around-statements,misc-macro-parentheses \
  *.c \
  -- -I.

