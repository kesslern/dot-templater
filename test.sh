#!/bin/bash
valgrind --track-origins=yes ./dot-templater rules dotfiles dest
