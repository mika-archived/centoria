# Centoria

Centoria - function manager for macOS and Linux.

## Usage

initialize or reload centoria:

```bash
# bash or zsh
$ source $(cet init)

# fish
$ source (cet init | psub)
```

Add a function that work as alias:

```bash
$ cet add search rg
```

Remove a function:

```bash
$ cet remove search
```

You can use conditional statement:

```bash
# if `which rg` returns success code (exit 0), you can use `search` command.
$ cet add search rg --condition "which rg"
```

If you want to pass the arguments anywhere, you can specify them using `{INDEXER}` as placeholder.  
Example:

- You want the 1st arguments, please use `{0}` for placeholder.
- You want the 1st, 2nd and 3rd arguments, please use `{0..3}` for placeholder.
- You want the 2nd and later arguments, please use `{2..}` for placeholder.
- You want the 2nd and later arguments, but they are optional, please use `{2..?}` for placeholder.
  - `?` acts as optional parameter
    - Single Optional : `{0?}`
    - Multiple Optional : `{0..?}`

```bash
# explicitly set the position of a parameter for search
$ cet add search "rg {0..}" --condition "which rg"
# "show-err-logs nginx" expands as "tail -f /var/log/nginx/error.log"
$ cet add show-err-logs "tail -f /var/log/{0}/error.log"
```

Centoria also supports functions as sub-command:

```bash
# `docker c` expands as `docker container`
$ cet add c container --program docker
# also use function (this function has no meaning, but an example)
$ cet add prune "{0} prune" --program docker
# remove
$ cet remove prune --program docker
```

Execute the function:

```bash
# direct (required `cet init`)
$ search "Hello" ./README.md
# via centoria
$ cet exec search -- "Hello" ./README.md
# subcommands (direct, required `cet init`)
$ docker c
# subcommands (via centoria)
$ cet exec docker -- c
```

If you want to use centoria as collection of subcommands:

```bash
# 1st, create a base command
$ cet add centoria "CENTORIA_CONFIG_PATH=/path/to/config.toml cet exec {0} -- {1..?}" --shell bash
# 2nd, write functions to /path/to/config.toml
$ vim /path/to/config.toml
# 3rd, execute via base command
$ centoria search "Hello" ./README.md
```

For more information about Centoria, please see the result of `cet help` or `cet help <COMMAND>`.


## Centoria TOML configuration

Centoria find configuration from the following paths:

- `$CENTORIA_CONFIG_PATH`
- System configuration directory
  - Linux: `$XDG_CONFIG_HOME/centoria/centoria.toml` or `$HOME/.config/centoria/centoria.toml`
  - macOS: `$HOME/Library/Preferences/centoria/centoria.toml`
  - Windows: `$APPDATA/centoria/centoria.toml`
- `$HOME/.centoria.toml`

If you add a new function from command-line, Centoria creates a new file in `$HOME/.centoria.toml`.

example `centoria.toml` :

```toml
[search]
runas = 'alias'
command = 'rg'
condition = 'which rg'

[show-err-logs]
runas = 'function'
command = 'tail -f /var/log/{0}/error.log'
description = 'show error logs'
descriptions = [
  'application name' # description of argument {0}
]

[docker]
runas = 'subcommand'
command = 'docker'

[docker.subcommands.c]
command = 'container'
```
