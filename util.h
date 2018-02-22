#pragma once

/** Determine if a file is a regular file. */
int is_file(const char *name);

/** Determine if a file is a directory. */
int is_dir(const char *name);

/** Read a whole file into a string. */
char *read_file(char *filename);

/** Returns a valid calloced memory pointer. Exits on failure. */
void *safe_calloc(size_t n, size_t size);

/** Find one occurance of key in str and replace with value in a new string. */
char *strsub(const char *str, const char *key, const char *value);
