#pragma once
#include <stdbool.h>

/** Determine if a file is a regular file. */
bool is_file(const char *name);

/** Determine if a file is a directory. */
bool is_dir(const char *name);

/** Read a whole file into a string. */
char *read_file(const char *filename);

/** Returns a valid calloced memory pointer. Exits on failure. */
void *safe_calloc(size_t n, size_t size);

/** Find one occurance of key in str and replace with value in a new string. */
char *strsub(const char *str, const char *key, const char *value);

/** Determine if a file is binary or not. */
bool is_binary_file(const char *fpath);

/** Copies a src file to a destination file.*/
void copy_file(const char *src, const char *dest);

/** Duplicates permissions on src to dest. */
void copy_permission(const char *src, const char *dest);
