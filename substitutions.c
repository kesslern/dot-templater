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

substitution *first_substitution = NULL;

/**
 * Callback function for saving a substitution.
 */
void substitution_saver(char *key, char *value)
{
    static substitution *current = NULL;
    if (current == NULL) {
        current = first_substitution = safe_calloc(1, sizeof(substitution));
    }

    current->key = key;
    current->value = value;
    current->next = safe_calloc(1, sizeof(substitution));
    current = current->next;
    current->next = NULL;
}

/**
 * Run substitutions through the provided line and provide a new string with the
 * result.
 */
char *substitute_line(char *line)
{
    // TODO: Make this method more readable
    substitution *current = first_substitution;
    char *result = safe_calloc(strlen(line) + 1, sizeof(char));
    memcpy(result, line, strlen(line) + 1);

    /* Make each available substitution in the line. */
    while (current != NULL && current->key != NULL) {
        // TODO: strstr is called here and in strsub
        while (strstr(result, current->key) != NULL) {
            char *new_line = strsub(result, current->key, current->value);
            free(result);
            result = new_line;
        }
        current = current->next;
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
    free_substitutions_list(first_substitution);
}
