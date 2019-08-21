# Centoria

Centoria - function manager for macOS and Linux.

## Usage

Initialize centoria:

```bash
# bash or zsh
$ source $(cet init)

# fish
$ source (cet init | psub)
```

Add a function:

```bash
$ cet add grep rg
```

Remove a function:

```bash
$ cet remove grep
```

Centoria supports conditional function:

```bash
# if `which rg` returns success code (exit 0), use `rg` instead of `grep`
$ cet add grep rg --condition "which rg"
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
runas = 'alias
command = 'rg'
condition = 'which rg'
```