#define _GNU_SOURCE
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "config.h"

#define IS_COMMENT(x) (*(x) == '#')

void substitution_parser(char *str, config *config)
{
    static char *current_key = NULL;
    static char *current_value = NULL;

    if (str != NULL) {
        if (current_key == NULL) {
            current_key = str;
        } else {
            if (current_value == NULL) {
                current_value = str;
            } else {
                *(str - 1) = '=';
            }
        }
    } else {
        (*config->substitution_saver)(current_key, current_value);
        current_key = NULL;
        current_value = NULL;
    }
}

void tokenizer(char *str, char *delims, void (*handler)(char *, config *),
               config *config)
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

void parse_line(char *str, config *config)
{
    if (str != NULL && !IS_COMMENT(str)) {
        tokenizer(str, "=", &substitution_parser, config);
    }
}

void parse_configuration(char *str, config config)
{
    tokenizer(str, "\n", &parse_line, &config);
}
