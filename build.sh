#!/bin/bash
gcc -std=c11 -O2 -Wall -Wextra -Wpedantic -Werror util.c config.c dottemplater.c -o dot-templater
