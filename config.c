#define _GNU_SOURCE
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "config.h"

void substitution_parser(char *str, config config)
{
    static int key_saved = false;
    static int value_saved = false;
    static char *current_key;
    static char *current_value;

    if (str != NULL) {
        if (!key_saved) {
            current_key = str;
            key_saved = true;
        } else {
            if (!value_saved) {
                current_value = str;
                value_saved = true;
            } else {
                *(str - 1) = '=';
            }
        }
    } else {
        (*config.substitution_saver)(current_key, current_value);
        key_saved = false;
        value_saved = false;
        current_key = NULL;
        current_value = NULL;
    }
}

void tokenizer(char *str, char *delims, void (*handler)(char *, config),
               config config)
{
    char *saveptr;
    char *pos;

    pos = strtok_r(str, delims, &saveptr);
    (*handler)(pos, config);

    do {
        pos = strtok_r(NULL, delims, &saveptr);
        (*handler)(pos, config);
    } while (pos != NULL);
}

void parse_line(char *str, config config)
{
    if (str != NULL && *str != '#') {
        tokenizer(str, "=", &substitution_parser, config);
    }
}

void parse_configuration(char *str, config config)
{
    tokenizer(str, "\n", &parse_line, config);
}
