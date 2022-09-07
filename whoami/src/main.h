#ifndef MAIN_H
#define MAIN_H
#define PROGRAM_NAME "whoami"
#define BUFFER 4024
#include <stdlib.h>
#include <pwd.h>
#include <stdio.h>
#include <unistd.h>
#include <argp.h>
#include <string.h>

char *get_username(void);
unsigned int get_uid(void);
static error_t parse_opt(int key, char *arg, struct argp_state *state);
static struct argp_option options[] = {
    {"uid", 'u', 0, 0, "Print uid instead of username"},
    {0}
    };

struct arguments
{
  unsigned short uid;
};
#endif