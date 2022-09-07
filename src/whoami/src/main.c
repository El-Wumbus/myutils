#include "main.h"
/* Program documentation. */

/* A description of the arguments we accept. */

int main(int argc, char **argv)
{
  static char doc[] = "whoami -- Print the user name associated with the current effective UID.";
  static char args_doc[] = "";

  struct arguments args;
  static struct argp argp = {options, parse_opt, args_doc, doc};
  args.uid = 0;
  argp_parse(&argp, argc, argv, 0, 0, &args);

  if (args.uid)
  {
    printf("%d\n", get_uid());
    return 0;
  }

  char *username = get_username();
  printf("%s\n", username);
  free(username);
  return 0;
}

unsigned int get_uid()
{
  register __uid_t uid = geteuid();
  if (uid)
  {
    return uid;
  }

  return 0;
}

char *get_username()
{
  register unsigned int uid = get_uid();

  register struct passwd *pw;
  pw = getpwuid(uid);

  if (uid && pw)
  {
    char *username = (char *)calloc(BUFFER, sizeof(char));
    strcpy(username, pw->pw_name);
    username = (char *)realloc(username, (strlen(username) + 1) * sizeof(char));
    return username;
  }

  char *empty = calloc(1, sizeof(char));
  empty[0] = '\0';
  return empty;
}

static error_t parse_opt(int key, char *arg, struct argp_state *state)
{
  /* Get the input argument from argp_parse, which we
     know is a pointer to our arguments structure. */
  struct arguments *arguments = state->input;

  switch (key)
  {
  case 'u':
    arguments->uid = 1;
    break;
  }
  return 0;
}
