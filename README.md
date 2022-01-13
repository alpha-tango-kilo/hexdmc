# HexDMC

`hexdmc` is a commandline tool to convert between DMC flosses and hex colours

## Installation

Requires Rust to have be installed.
Otherwise, you can check out the [releases page](https://codeberg.org/alpha-tango-kilo/hexdmc/releases) for any pre-compiled executables

```shell
cargo install --git https://codeberg.org/alpha-tango-kilo/hexdmc
```

To update, add the `--force` flag to the above command

## Usage

You use a subcommand to specify the type of the colour(s) you're passing; either `dmc` or `hex`, then list any colours, separated by spaces.
You can't mix & match colour types in a single command

```
hexdmc <dmc|hex> [COLOUR] ...
```

## Limitations (a note about hex colour conversion)

Hex colours don't always exactly correspond to a DMC colour.
In this case, HexDMC will find the closest* matching colour(s).
The output arrow will show as `~>` if the result is only approximate (as opposed to `->` usually)
