#define _GNU_SOURCE
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "config.h"

#define IS_COMMENT(x) (*(x) == '#')

void parse_line(char *str, config *config)
{
    if (str != NULL && !IS_COMMENT(str)) {
        char *split = strchr(str, '=');
        if (split != NULL) {
            *split = '\0';
            (*config->substitution_saver)(str, ++split);
        } else {
            (*config->feature_saver)(str);
        }
    }
}

void parse_configuration(char *str, config config)
{
    char *saveptr;
    char *pos;

    pos = strtok_r(str, "\n", &saveptr);
    parse_line(pos, &config);

    do {
        pos = strtok_r(NULL, "\n", &saveptr);
        parse_line(pos, &config);
    } while (pos != NULL);
}
