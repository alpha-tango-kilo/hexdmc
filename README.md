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

## Limitations

### Hex colour conversion

Hex colours don't always exactly correspond to a DMC colour.
In this case, HexDMC will find the closest* matching colour(s).
The output arrow will show as `~>` if the result is only approximate (as opposed to `->` usually)

### Terminal colour support

HexDMC shows a blob of each colour in its output.
**Please do not consider this colour to be of true likeness**.
Terminals are honestly pretty crap at supporting a wide range of colours, so just use the feature as a general indication.
If you know your terminal supports full 8-bit colour, then it should be of true likeness, assuming the [output colouring library](https://lib.rs/crates/owo-colors) I'm using supports it
