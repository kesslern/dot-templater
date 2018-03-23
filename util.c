#include <fcntl.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

#include "util.h"

#define BUFFER_SIZE 8192

void *safe_calloc(size_t n, size_t size)
{
    void *p = calloc(n, size);
    if (p == NULL) {
        perror("Unable to allocate memory.\n");
        exit(1);
    }
    return p;
}

int filesize(const char *name)
{
    struct stat s;
    if (stat(name, &s) != 0) {
        return 0;
    };
    return s.st_size;
}

bool is_file(const char *name)
{
    struct stat s;
    if (stat(name, &s) != 0) {
        return 0;
    };
    return S_ISREG(s.st_mode);
}

bool is_dir(const char *name)
{
    struct stat s;
    if (stat(name, &s) != 0) {
        return 0;
    };
    return S_ISDIR(s.st_mode);
}

char *read_file(const char *filename)
{
    FILE *file = fopen(filename, "rb");
    char *file_buffer;

    if (file != NULL) {
        long size = filesize(filename);
        file_buffer = safe_calloc(size + 1, sizeof(char));
        fread(file_buffer, size, 1, file);
        fclose(file);
    } else {
        file_buffer = safe_calloc(1, sizeof(char));
    }

    return file_buffer;
}

char *strsub(const char *str, const char *key, const char *value)
{
    /* How far into the string the key occurrance is. */
    int occurance_len = strstr(str, key) - str;
    int value_len = strlen(value);
    int key_len = strlen(key);
    int str_len = strlen(str);
    int new_str_len = str_len + value_len - key_len;
    /* Account for null terminator when allocating length */
    char *new_str = safe_calloc(new_str_len + 1, sizeof(char));

    /* Copy up to the occurrance of the key. */
    memcpy(new_str, str, occurance_len);
    /* Copy the value into the new string at the occurance location. */
    memcpy(new_str + occurance_len, value, value_len);
    /* Copy the rest of the string. */
    memcpy(new_str + occurance_len + value_len, str + occurance_len + key_len,
           str_len - occurance_len - key_len);

    return new_str;
}

/*
 * Implementation is copied from git.
 * https://bit.ly/2DMtwBp
 */
#define FIRST_FEW_BYTES 8000
bool buffer_is_binary(const char *ptr, unsigned long size)
{
    if (FIRST_FEW_BYTES < size) {
        size = FIRST_FEW_BYTES;
    }
    return !!memchr(ptr, 0, size);
}

bool is_binary_file(const char *fname)
{
    FILE *file = fopen(fname, "rb");
    char *file_buffer;
    bool result = false;

    if (file != NULL) {
        long size = filesize(fname);
        file_buffer = safe_calloc(size + 1, sizeof(char));
        fread(file_buffer, size, 1, file);
        result = buffer_is_binary(file_buffer, size);
        fclose(file);
        free(file_buffer);
    }

    return result;
}

void copy_file(const char *src, const char *dest)
{
    /* Input and output file descriptors */
    int input_file, output_file;
    /* Number of bytes returned by read() and write() */
    ssize_t read_bytes, write_bytes;
    /* Character buffer */
    char buffer[BUFFER_SIZE];

    input_file = open(src, O_RDONLY);
    if (input_file == -1) {
        perror("open");
        exit(1);
    }

    output_file = open(dest, O_WRONLY | O_CREAT, 0644);
    if (output_file == -1) {
        perror("open");
        exit(1);
    }

    while ((read_bytes = read(input_file, &buffer, BUFFER_SIZE)) > 0) {
        write_bytes = write(output_file, &buffer, (ssize_t)read_bytes);
        if (write_bytes != read_bytes) {
            perror("write");
            exit(1);
        }
    }

    /* Close file descriptors */
    close(input_file);
    close(output_file);
}

void copy_permission(const char *src, const char *dest)
{
    struct stat s;
    stat(src, &s);

    if (chmod(dest, s.st_mode) < 0) {
        perror("chmod");
        exit(1);
    }
}
