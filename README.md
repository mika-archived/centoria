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

* You want the 1st arguments, please use `{0}` for placeholder.
* You want the 1st, 2nd and 3rd arguments, please use `{0..3}` for placeholder.
* You want the 2nd and later arguments, please use `{2..}` for placeholder.
* You want the 2nd and later arguments, but they are optional, please use `{2..?}` for placeholder.
  * `?` acts as optional parameter
    * Single Optional : `{0?}`
    * Multiple Optional : `{0..?}`


```bash
# explicitly set the position of a parameter for search
$ cet add search "rg {0..}" --condition "which rg"
# "show-err-logs nginx" expands as "tail -f /var/log/nginx/error.log"
$ cet add show-err-logs "tail -f /var/log/{0}/error.log"
```


## Centoria TOML configuration

Centoria find configuration from the following paths:

* `$CENTORIA_CONFIG_PATH`
* System configuration directory
  * Linux: `$XDG_CONFIG_HOME/centoria/centoria.toml` or `$HOME/.config/centoria/centoria.toml`
  * macOS: `$HOME/Library/Preferences/centoria/centoria.toml`
  * Windows: `$APPDATA/centoria/centoria.toml`
* `$HOME/.centoria.toml`

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
```

