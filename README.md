# Centoria

Centoria - alias manager for macOS and Linux.

## Usage

Initialize centoria:

```bash
# bash or zsh
$ source $(cet init)

# fish
$ source (cet init | psub)
```

Add a alias:

```bash
$ cet add grep rg
```

Remove a alias:

```bash
$ cet remove grep
```

Centoria supports conditional alias:

```bash
# if `which rg` returns success code (exit 0), use `rg` instead of `grep`
$ cet add grep rg --condition "which rg"
```

## Centoria TOML configuration

Centoria find configuration from the following paths:

* `$CENTORIA_CONFIG_PATH`
* `$XDG_CONFIG_HOME/centoria/centoria.toml`
* `$HOME/.centoria.toml`

example `centoria.toml` :

```toml
[grep]
command = "rg"
condition = "which rg"
```