# Test Command: Unknown

`pilum unknown` is an unknown command and therefore returns an error with exitcode `2`:

```console
$ pilum unknown
? 2
error: unrecognized subcommand 'unknown'

Usage: pilum [COMMAND]

For more information, try '--help'.

```

`pilum` supplies no command whatsoever and therefore returns an error with exitcode `2`:

```console
$ pilum
? 2
error: no subcommand specified

Usage: pilum [COMMAND]

For more information, try '--help'.

```
