/** Callback function for saving the name of an enabled feature. */
void feature_saver(char *feature);

/** Release all features from memory. */
void free_features();

/** Determines if a given feature is enabled. */
bool is_feature_enabled(char *feature);

/** Determines if a line is a feature enable or disable line. */
bool is_feature_enable_or_disable(char *line);
