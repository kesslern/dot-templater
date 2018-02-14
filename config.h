#pragma once

typedef struct parse_config_t {
  void (*substitution_saver)(char *, char *);
} config;

void parse_configuration(char *str, config config);
