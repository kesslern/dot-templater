/** Callback function for saving a substitution. */
void substitution_saver(char *key, char *value);

/**
 * Run substitutions through the provided line and provide a new string with the
 * result.
 */
char *substitute_line(char *line);

/** Frees all allocated substitutions. */
void free_substitutions();
