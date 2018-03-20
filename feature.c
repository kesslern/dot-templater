#include <regex.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "feature.h"
#include "util.h"

/**
 * Stores the name of an enabled feature.
 */
typedef struct feature_t {
    char *feature_name;
    struct feature_t *next;
} feature;

feature *first_feature = NULL;

void feature_saver(char *feature)
{
    static struct feature_t *current = NULL;
    if (current == NULL) {
        current = first_feature = safe_calloc(1, sizeof(struct feature_t));
    }
    current->feature_name = feature;
    current->next = safe_calloc(1, sizeof(struct feature_t));
    current = current->next;
}

/**
 * Frees all allocated features in the list.
 */
void _free_features(feature *f)
{
    if (f != NULL) {
        _free_features(f->next);
        free(f);
    }
}

void free_features()
{
    _free_features(first_feature);
}

bool _is_feature_enabled(feature *features, char *feature)
{
    char *last_char = feature + strlen(feature) - 1;
    if (*last_char == '\n') {
        *last_char = '\0';
    }
    if (features->feature_name != NULL) {
        if (strcmp(features->feature_name, feature) == 0) {
            return true;
        }
        return _is_feature_enabled(features->next, feature);
    }
    return false;
}

bool is_feature_enabled(char *feature)
{
    return _is_feature_enabled(first_feature, feature);
}

bool is_feature_enable_or_disable(char *line)
{
    regex_t r;
    regcomp(&r, "^\\s*### .*$", REG_EXTENDED | REG_NOSUB);
    bool ret = regexec(&r, line, 0, 0, 0) == 0;
    regfree(&r);
    return ret;
}
