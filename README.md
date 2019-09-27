<div align="center">
<h1>bishop.rs</h1>

<a href="https://travis-ci.org/AbsurdlySuspicious/bishop.rs">
<img src="https://travis-ci.org/AbsurdlySuspicious/bishop.rs.svg?branch=master">
</a>

<table><tr><td><pre>
+----[drunken]----+
|..+.. oo+o.      |
| *.* o +.o.      |
|= = o * .E+      |
|+. ..+ = +..     |
|o  ...+ S.o      |
| .o .....= .     |
|.. o   .+ +      |
|.        = +     |
|        ..=      |
+----[bishop]-----+
</pre></td></tr></table>

</div>

Library and CLI app for visualizing data using The Drunken Bishop algorithm implemented in Rust

> Drunken Bishop is the algorithm used in OpenSSH's `ssh-keygen` for visualising generated keys 

**Table of Contents:**

* [Crates](#crates)
* [Install](#install)
* [Examples](#examples)
	* [Using as command-line app](#using-as-command-line-app)
	* [Using as library](#using-as-library)
		* [`Cargo.toml`](#cargotoml)
		* [For `AsRef<u8>` (slices, vectors)](#for-asrefu8-slices-vectors)
		* [Drawing options and result reusing](#drawing-options-and-result-reusing)
		* [For `Read` (file, stdin, etc)](#for-read-file-stdin-etc)
* [License](#license)
	* [Contribution](#contribution)

## Crates

Crate                              | Description      | Version                                                                                                                                                 |
-----------------------------------|------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------|
bishop                             | Library          | [![cargo](https://img.shields.io/crates/v/bishop)](https://crates.io/crates/bishop) [![docs](https://docs.rs/bishop/badge.svg)](https://docs.rs/bishop) |
bishop-cli ([source](bishop-cli/)) | Command-line app | [![cargo](https://img.shields.io/crates/v/bishop-cli)](https://crates.io/crates/bishop-cli)                                                             |

## Install

**Rust library** lives on [crates.io](https://crates.io/crates/bishop)

**CLI app:**

Platform                | Package                                                                                        |
------------------------|------------------------------------------------------------------------------------------------|
**Arch Linux**          | [![aur](https://img.shields.io/aur/version/bishop)](https://aur.archlinux.org/packages/bishop) |
**Prebuilts for Linux** | [Github releases](https://github.com/AbsurdlySuspicious/bishop.rs/releases)                    |

## Examples

### Using as command-line app

```bash
some_data=$(printf foobar | sha256sum | cut -d' ' -f1)
# we are using cut here to crop the filename from sha256sum output

printf $some_data | bishop -sI hex
# `-s` tells bishop to take data from stdin
#
# `-I hex` tells bishop that input data will be in HEX format.
# As an alternative, you might use xxd to turn hex data into binary:

printf $some_data | xxd -r -p | bishop -s
# `-I bin` is implied by default

bishop -i <(printf $some_data) -I hex
# `-i` tells bishop to take data from specified file.
# We are using bash command substitution here, but
# any valid path is allowed, like `bishop -i ~/some.file`

bishop $some_data
# Without `-i` or `-s` bishop expects HEX encoded input in the first argument.
# Note that `-I` is not supported if data is provided as argument

printf foobar | bishop -sI hash
# `-I hash` tells bishop to hash all of its input
# using sha256 before making a randomart.
# Since maximum effective size of input data for random art with default size (17x9)
# is somwhere around 64-128 bytes, this option is extremely useful for large inputs  
```

All these bishop calls would print this art to console:

```
Fingerprint of:
c3ab8ff13720e8ad9047dd39466b3c8974e592c2fa383d4a3960714caef0c4f2

+-----------------+
|    .     .      |
| . + .   +       |
|o + + + = .      |
| * + + O =       |
|  E o.o S        |
| . +.=.o.=       |
|  o.B.=...       |
|   +.+.*  o      |
|    o.o.o. .     |
+-----------------+
``` 

Note that input will be echoed only if data is provided as argument or with `-I hash`.
This behavior can be disabled using `-q` option.

You can read full usage for cli app (also available by `--help` option)
[here](bishop-cli/usage.txt)

#### Note on char list

You can provide custom char list wich `-c` option.
This is a vector of chars used for fingerprint (represened as string).

Each char is treated as:

Index  | Description             | Default          |
-------|-------------------------|------------------|
`0`    | Field background        | ` `              |
`1..n` | Chars used for drawing  | `.o+=*BOX@%&#/^` |
`n+1`  | Char for start position | `S`              |
`n+2`  | Char for last position  | `E`              |

Each non-background char indicates how many
times bishop has been on this position.

Start and end chars overwrites the real value.

Char list must be at least 4 chars long,
but secure char list is at least 18 chars long
and only consists of clearly distinguishable symbols.

### Using as library

#### `Cargo.toml`

```toml
bishop = "0.2.0"
```

Use latest version as stated on cargo badge [above](#Crates)

#### For `AsRef<u8>` (slices, vectors)

```rust
extern crate bishop;
use bishop::*;

fn main() {
    let data1 = [0u8; 16];
    let data2 = vec![0u8; 16];

    let mut art = BishopArt::new();
    art.input(&data1);
    art.input(&data2);
    println!("{}", art.draw());

    // Using chaining:

    let drawn_art: String = BishopArt::new()
        .chain(&data1)
        .chain(&data2)
        .draw();
    println!("{}", drawn_art);
}
```

#### Drawing options and result reusing

```rust
use bishop::*;

fn random_art(data: &[u8]) {
    let opts1 = DrawingOptions { top_text: "pass 1".to_string(), ..Default::default() };
    let opts2 = DrawingOptions { bottom_text: "pass 2".to_string(), ..Default::default() };

    // compute field once
    let field = BishopArt::new().chain(data).result();

    // then draw it multiple times with different options
    println!("{}", field.draw_with_opts(&opts1));
    println!("{}", field.draw_with_opts(&opts2));
}
```

#### For `Read` (file, stdin, etc)

```rust
use bishop::*;
use std::io::{self, Read};

fn main() {
    // BishopArt implements Write trait
    let mut art = BishopArt::new();
    io::copy(&mut io::stdin(), &mut art);
    println!("{}", art.draw());
}
```

Full API documentation is available on [docs.rs](https://docs.rs/bishop)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
