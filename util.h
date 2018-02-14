#pragma once

long is_file(char *name);
long is_dir(char *name);
char *read_file(char *filename);
void *safe_calloc(size_t n, size_t size);
