#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

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

int is_file(const char *name)
{
    struct stat s;
    if (stat(name, &s) != 0) {
        return 0;
    };
    return S_ISREG(s.st_mode);
}

long is_dir(char *name)
{
    struct stat s;
    if (stat(name, &s) != 0) {
        return 0;
    };
    return S_ISDIR(s.st_mode);
}

char *read_file(char *filename)
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
