#define _GNU_SOURCE
#include <ftw.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "config.h"
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

substitution *first_substitution;

/**
 * Stores the name of an enabled feature.
 */
typedef struct feature_t {
    char *feature_name;
    struct feature_t *next;
} feature;

feature *first_feature;

/**
 * Callback function for saving the name of an enabled feature.
 */
void feature_saver(char *feature)
{
    static struct feature_t *current = NULL;
    if (current == NULL) {
        current = first_feature;
    }
    printf("Saving feature %s\n", feature);
    current->feature_name = feature;
    current->next = safe_calloc(1, sizeof(struct feature_t));
    current = current->next;
}

/**
 * Callback function for saving a substitution.
 */
void substitution_saver(char *key, char *value)
{
    static substitution *current = NULL;
    if (current == NULL) {
        current = first_substitution;
    }
    printf("Saving substitution %s=%s\n", key, value);
    current->key = key;
    current->value = value;
    current->next = safe_calloc(1, sizeof(substitution));
    current = current->next;
    current->next = NULL;
}

/**
 * Frees all allocated features in the list.
 */
void free_features(feature *f)
{
    if (f != NULL) {
        free_features(f->next);
        free(f);
    }
}

/**
 * Frees all allocated substitutions in the list.
 */
void free_substitutions(substitution *sub)
{
    if (sub != NULL) {
        free_substitutions(sub->next);
        free(sub);
    }
}

/**
 * Find one occurance of key in str and replace with value in a new string.
 */
char *strsub(const char *str, const char *key, const char *value)
{
    int occurance_len = strstr(str, key) - str;
    int value_len = strlen(value);
    int key_len = strlen(key);
    int str_len = strlen(str);
    int new_len = str_len + value_len - key_len;
    char *new_str = safe_calloc(new_len + 1, sizeof(char));

    memcpy(new_str, str, occurance_len);
    memcpy(new_str + occurance_len, value, value_len);
    memcpy(new_str + occurance_len + value_len, str + occurance_len + key_len,
           str_len - occurance_len - key_len);

    return new_str;
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

/**
 * Reads the file at path [input] and writes it to [output], substituting
 * values on each line.
 */
void substitute_file(const char *input, const char *output)
{
    FILE *in, *out;
    char *line = NULL;
    char *new_line;
    size_t len = 0;

    in = fopen(input, "r");
    out = fopen(output, "w");

    if (in == NULL || out == NULL) {
        perror("fopen");
        exit(EXIT_FAILURE);
    }

    while (getline(&line, &len, in) != -1) {
        new_line = substitute_line(line);
        fwrite(new_line, sizeof(char), strlen(new_line), out);
        free(new_line);
    }

    free(line);
    fclose(in);
    fclose(out);
}

char *source_dir;
char *dest_dir;

/**
 * Callback function for nftw. For directories, it is ensured that a
 * corresponding directory in the destination path exists. Files are copied to
 * the destination after substituting values. Uses the global values
 * [source_dir] and [dest_dir].
 */
int walker(const char *fpath, __attribute__((unused)) const struct stat *sb,
           __attribute__((unused)) int flags, struct FTW *ftwbuf)
{
    char *dest_file = strsub(fpath, source_dir, dest_dir);
    printf("%s | %s | %s ", fpath, fpath + ftwbuf->base, dest_file);
    if (is_dir(fpath)) {
        printf("directory ");
        if (is_dir(dest_file)) {
            printf("dest exists ");
        } else {
            printf("dest does not exist ");
            mkdir(dest_file, 0700);
        }
    } else {
        substitute_file(fpath, dest_file);
    }

    printf("\n");
    free(dest_file);
    return 0;
}

void print_help()
{
    printf("usage: dot-templater RULES SRC_DIR DEST_DIR\n");
    printf("\n");
    printf("Copies files in SRC_DIR to DEST_DIR using rules in RULES.\n");
    printf("\n");
    printf("Rules configuration:\n");
    printf("  Keys and values are separated by the first occurance of '=' in a "
           "line.\n");
    printf("  In each copied file, each key is replaced by the associated "
           "value.\n");
}

int main(int argc, char **argv)
{
    if (argc != 4) {
        print_help();
        exit(EXIT_FAILURE);
    }

    if (!is_file(argv[1])) {
        printf("Expected first argument to be a rules file.\n\n");
        print_help();
        exit(EXIT_FAILURE);
    }

    if (!is_dir(argv[2])) {
        printf("Expected second argument to be a dotfiles directory.\n\n");
        print_help();
        exit(EXIT_FAILURE);
    }

    if (!is_dir(argv[3])) {
        printf("Expected third argument to be a destination directory.\n\n");
        print_help();
        exit(EXIT_FAILURE);
    }

    source_dir = argv[2];
    dest_dir = argv[3];

    first_substitution = safe_calloc(1, sizeof(substitution));
    first_feature = safe_calloc(1, sizeof(feature));

    /* Parse rules file with variable substitutions. */
    char *buffer = read_file(argv[1]);
    config config = {.substitution_saver = &substitution_saver,
                     .feature_saver = &feature_saver};
    parse_configuration(buffer, config);

    /* Walk the source directory and save in the destination directory. */
    nftw(argv[2], &walker, 15, 0);

    /* Cleanup. */
    free(buffer);
    free_substitutions(first_substitution);
    free_features(first_feature);
    exit(EXIT_SUCCESS);
}
