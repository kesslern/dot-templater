#define _GNU_SOURCE
#include <ftw.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "config.h"
#include "util.h"

typedef struct substitution_t {
    char *key;
    char *value;
    struct substitution_t *next;
} substitution;

substitution *first;

void substitution_saver(char *key, char *value)
{
    static substitution *current = NULL;
    if (current == NULL) {
        current = first;
    }
    current->key = key;
    current->value = value;
    current->next = safe_calloc(1, sizeof(substitution));
    current = current->next;
}

void free_substitutions(substitution *sub)
{
    if (sub != NULL) {
        if (sub->next != NULL) {
            free_substitutions(sub->next);
        }
        free(sub);
    }
}

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

char *substitute_line(char *line)
{
    substitution *current = first;
    char *result = NULL;

    while (current != NULL && current->key != NULL) {
        if (strstr(line, current->key) != NULL) {
            result = strsub(line, current->key, current->value);
        }
        current = current->next;
    }

    if (result == NULL) {
        result = safe_calloc(strlen(line), sizeof(char) + 1);
        memcpy(result, line, sizeof(char) * (strlen(line) + 1));
    }

    return result;
}

void substitute_file(const char *input, const char *output)
{
    FILE *in, *out;
    char *line = NULL;
    char *new_line = NULL;
    size_t len = 0;
    ssize_t nread;

    in = fopen(input, "r");
    out = fopen(output, "w");

    if (in == NULL || out == NULL) {
        perror("fopen");
        exit(EXIT_FAILURE);
    }

    while ((nread = getline(&line, &len, in)) != -1) {
        new_line = substitute_line(line);
        printf("%s", new_line);
        fwrite(new_line, sizeof(char), strlen(new_line), out);
        free(new_line);
    }

    free(line);
    fclose(in);
    fclose(out);
}

char *source_dir;
char *dest_dir;

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

int main(int argc, char **argv)
{
    if (argc != 4) {
        printf("Expected 3 arguments.\n");
        exit(EXIT_FAILURE);
    }

    if (!is_file(argv[1])) {
        printf("Expected first argument to be a rules file.\n");
    }

    if (!is_dir(argv[2])) {
        printf("Expected second argument to be a dotfiles file.\n");
        exit(EXIT_FAILURE);
    }

    if (!is_dir(argv[3])) {
        printf("Expected third argument to be a destination directory.\n");
        exit(EXIT_FAILURE);
    }

    source_dir = argv[2];
    dest_dir = argv[3];

    /* Parse rules file with variable substitutions. */
    char *buffer = read_file(argv[1]);
    config config = {.substitution_saver = &substitution_saver};
    first = safe_calloc(1, sizeof(substitution));
    parse_configuration(buffer, config);

    // char *template = read_file(argv[2]);
    // substitute_file(argv[1], "out");

    nftw(argv[2], &walker, 15, 0);
    printf("ok\n");
    free(buffer);
    // free(template);
    free_substitutions(first);
    exit(EXIT_SUCCESS);
}
