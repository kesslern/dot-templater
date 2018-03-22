#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "substitutions.h"
#include "util.h"

/**
 * Stores key/value pairs of strings that will be substituted in the copied
 * files in a single-linked list.
 */
typedef struct substitution_t {
    char *key;
    char *value;
    struct substitution_t *next;
} substitution;

substitution *substitutions_head = NULL;

/**
 * Callback function for saving a substitution.
 */
void substitution_saver(char *key, char *value)
{
    static substitution *last_substitution;
    static bool substitutions_initialized = false;
    substitution *new_substitution;

    new_substitution = safe_calloc(1, sizeof(substitution));
    new_substitution->key = key;
    new_substitution->value = value;

    if (!substitutions_initialized) {
        substitutions_head = last_substitution = new_substitution;
        substitutions_initialized = true;
    } else {
        last_substitution->next = new_substitution;
        last_substitution = new_substitution;
    }
}

/**
 * Run substitutions through the provided line and provide a new string with the
 * result.
 */
char *substitute_line(char *line)
{
    char *result = safe_calloc(strlen(line) + 1, sizeof(char));
    memcpy(result, line, strlen(line) + 1);

    /* Make each available substitution in the line. */
    for (substitution *substitution = substitutions_head; substitution != NULL;
         substitution = substitution->next) {
        /* Substitute each occurrence of the key. */
        while (strstr(result, substitution->key) != NULL) {
            /* Replace result with a substituted line. */
            char *new_line =
                strsub(result, substitution->key, substitution->value);
            free(result);
            result = new_line;
        }
    }

    /* No substitutions? Create a copy of the original string. */
    if (result == NULL) {
        result = safe_calloc(strlen(line) + 1, sizeof(char));
        memcpy(result, line, sizeof(char) * (strlen(line) + 1));
    }

    return result;
}

void free_substitutions_list(substitution *sub)
{
    if (sub != NULL) {
        free_substitutions_list(sub->next);
        free(sub);
    }
}

void free_substitutions()
{
    free_substitutions_list(substitutions_head);
}
