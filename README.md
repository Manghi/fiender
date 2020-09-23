# Fiender
[![Crate](https://img.shields.io/crates/v/fiender.svg)](https://crates.io/crates/fiender)
Fiender is a CLI tool written in Rust and can be used to retreive DnD5e spells and monsters stats. The only backend currently supported is https://open5e.com/.

## Installation
Install fiender using `cargo install fiender`, or clone this repository and build it using `cargo build`.

## Usage
Fiender will retrieve the specified spell/monster data and output it as formatted markdown.

To retrieve a spell or monster, use the `-s` or `-m` flags, respectively.
```Rust
$ fiender -s "eldritch blast"
$ fiender -m kobold
```

## Limitations
The spell/monster name must exactly match the entry in open5e.

## Aspirations
- Adding a fuzzy search/selection to eliminate the limitation described above.
- Support other output formats.
- Support more than just monsters and spells as Open5e is pretty cool!

## Remark
I use this for my own purposes and decided to make it public. Hope you find it useful!

# License
Fiender is distributed under the terms of the GPL (Version 3.0) license.

See [LICENSE](LICENSE) for details.
