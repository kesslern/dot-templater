#pragma once

typedef struct parse_config_t {
  void (*substitution_saver)(char *, char *);
} config;

/**
 * Parses a an entire configuration file contained in [str] according[<0;95;16M] to
 * [config].
 */
void parse_configuration(char *str, config config);
