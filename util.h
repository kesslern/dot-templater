#pragma once

int is_file(const char *name);
int is_dir(const char *name);
char *read_file(char *filename);
void *safe_calloc(size_t n, size_t size);
