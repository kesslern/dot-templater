#define _GNU_SOURCE
#include <ftw.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

#include "config.h"
#include "feature.h"
#include "substitutions.h"
#include "util.h"

/* These have to be global because they're used by the nftw callback. */
char *source_dir;
char *dest_dir;

/**
 * Reads the file at path [input] and writes it to [output], substituting
 * values on each line.
 */
void template_file(const char *input, const char *output)
{
    FILE *in, *out;
    char *line = NULL;
    char *new_line;
    size_t len = 0; // Unused but required by getline call
    int in_disabled_feature = false;

    in = fopen(input, "r");
    out = fopen(output, "w");

    if (in == NULL || out == NULL) {
        perror("fopen");
        exit(EXIT_FAILURE);
    }

    while (getline(&line, &len, in) != -1) {
        if (is_feature_enable_or_disable(line)) {
            if (in_disabled_feature) {
                in_disabled_feature = false;
            } else {
                in_disabled_feature =
                    !is_feature_enabled(strchr(line, '#') + 4);
            }
        } else if (!in_disabled_feature) {
            new_line = substitute_line(line);
            fwrite(new_line, sizeof(char), strlen(new_line), out);
            free(new_line);
        }
    }

    free(line);
    fclose(in);
    fclose(out);
}

/**
 * Callback function for nftw. For directories, it is ensured that a
 * corresponding directory in the destination path exists. Files are copied to
 * the destination after substituting values. Uses the global values
 * [source_dir] and [dest_dir].
 */
int walker(const char *fpath, const struct stat *sb,
           __attribute__((unused)) int flags,
           __attribute__((unused)) struct FTW *ftwbuf)
{
    char *dest_file = strsub(fpath, source_dir, dest_dir);

    if (S_ISDIR(sb->st_mode)) {
        if (!is_dir(dest_file)) {
            mkdir(dest_file, 0700);
        }
    } else {
        if (!is_binary_file(fpath)) {
            template_file(fpath, dest_file);
        } else {
            copy_file(fpath, dest_file);
        }
    }

    free(dest_file);
    return 0;
}

void print_help(char *name)
{
    printf("usage: %s RULES SRC_DIR DEST_DIR\n", name);
    printf("\n");
    printf("Copies files in SRC_DIR to DEST_DIR using rules in RULES.\n");
    printf("\n");
    printf("Rules configuration:\n");
    printf("  Keys and values are separated by the first occurance of '=' in a "
           "line.\n");
    printf("  In each copied file, each key is replaced by the associated "
           "value.\n");
    printf("\n");
    printf("Feature configuration:\n");
    printf("  Any line in the rules file without an '=' character is an "
           "enabled feature.\n");
    printf("  Features can be defined in dotfiles with three octothorpes "
           "followed by a\n");
    printf("  space and the feature name. Features that are not enabled in the "
           "rules will\n");
    printf("  be excluded in the templated file.\n");
}

int main(int argc, char **argv)
{
    if (argc != 4) {
        print_help(argv[0]);
        exit(EXIT_FAILURE);
    }

    if (!is_file(argv[1])) {
        printf("Expected first argument to be a rules file.\n\n");
        print_help(argv[0]);
        exit(EXIT_FAILURE);
    }

    if (!is_dir(argv[2])) {
        printf("Expected second argument to be a dotfiles directory.\n\n");
        print_help(argv[0]);
        exit(EXIT_FAILURE);
    }

    if (!is_dir(argv[3])) {
        printf("Expected third argument to be a destination directory.\n\n");
        print_help(argv[0]);
        exit(EXIT_FAILURE);
    }

    source_dir = argv[2];
    dest_dir = argv[3];

    /* Parse rules file with variable substitutions. */
    char *buffer = read_file(argv[1]);
    config config = {.substitution_saver = &substitution_saver,
                     .feature_saver = &feature_saver};
    parse_configuration(buffer, config);

    /* Walk the source directory and save in the destination directory. */
    nftw(argv[2], &walker, 15, 0);

    /* Cleanup. */
    free(buffer);
    free_substitutions();
    free_features();
    exit(EXIT_SUCCESS);
}
