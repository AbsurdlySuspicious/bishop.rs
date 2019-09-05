<div align="center">
<h1>drunken-bishop.rs</h1>

<a href="https://travis-ci.org/AbsurdlySuspicious/drunken-bishop.rs"><img src="https://travis-ci.org/AbsurdlySuspicious/drunken-bishop.rs.svg?branch=master"></a>
<!--badges-->

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

Implementation of The Drunken Bishop visual fingerprint algorithm
(that one used for so-called *randomarts* in **ssh-keygen**) in Rust.
This package provides library crate and command-line application for visualizing any binary or hex-formatted data.

Reference used for this implementation:
http://www.dirk-loss.de/sshvis/drunken_bishop.pdf

## Examples

### Using as command-line utility

```
$ drunken-bishop --help
drunken-bishop 0.1.0
Visualizes keys and hashes using OpenSSH's Drunken Bishop algorithm

USAGE:
    drunken-bishop [FLAGS] [OPTIONS] [hex]

FLAGS:
    -q, --quiet      Don't echo hex input
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i <input>               Input file; use '-' for stdin
        --chars <chars>      Custom char list: '[bg][char]...[start][end]'
    -w, --width <width>      Field width [default: 17]
    -h, --height <height>    Field height [default: 9]
    -t, --top <top>          Top frame text
    -b, --bot <bot>          Bottom frame text

ARGS:
    <hex>    Hex input; should have even length

$ hash=$(echo foobar | sha256sum | cut -d' ' -f1)
$ drunken-bishop "$hash"
Fingerprint of:
aec070645fe53ee3b3763059376134f058cc337247c978add178b6ccdfb0019f

+-----------------+
|          . .=*++|
|         o  o=@=*|
|    o   . . .*=@.|
|   o . . .  . E+ |
|  . . . S +o . =o|
|   +   . .+o  . o|
|    o   . oo     |
|     . .  .o.    |
|      .  ...     |
+-----------------+
$ echo "$hash" | xxd -r -p | drunken-bishop -i -
+-----------------+
|          . .=*++|
|         o  o=@=*|
|    o   . . .*=@.|
|   o . . .  . E+ |
|  . . . S +o . =o|
|   +   . .+o  . o|
|    o   . oo     |
|     . .  .o.    |
|      .  ...     |
+-----------------+
$ echo "$hash" | xxd -r -p > foobar.bin
$ drunken-bishop -i foobar.bin
+-----------------+
|          . .=*++|
|         o  o=@=*|
|    o   . . .*=@.|
|   o . . .  . E+ |
|  . . . S +o . =o|
|   +   . .+o  . o|
|    o   . oo     |
|     . .  .o.    |
|      .  ...     |
+-----------------+

```

### Using as library in my project

Add latest version of library from crates.io to your Cargo.toml, and then:

```rust
use drunken_bishop::bishop as bs;

use std::io::{Read, BufReader};
use std::fs::File;

fn main() {
    let cfg = bs::Options::default();

    // from file to String
    let file = File::open("some_file").unwrap();
    let art = bs::art_str(&mut BufReader::new(file).bytes(), &cfg).unwrap();
    println!("{}", art);

    // from slice to stdout
    let src: Vec<u8> = vec![1, 2, 3, 4, 5];
    bs::art_print(src.as_slice(), &cfg, |p| println!("{}", p)).unwrap();
}

```

## Options explanation

CLI        | Options struct       | Description                                 |
-----------|----------------------|---------------------------------------------|
`-w`, `-h` | `field_w`, `field_h` | Field size in range from 5,5 to 500,500     |
`-t`, `-b` | `top_str`, `bot_str` | Text to put on top and bottom frame border  |
`--chars`  | `chars`              | Chars that will be used for art (char list) |
`-i`       | -                    | Input file or `-` for stdin                 |

### Char list

Char list is a string each char of which is treated as:

Index  | Description             | Default          |
-------|-------------------------|------------------|
`1`    | Field background        | ` `              |
`2..n` | Chars used for drawing  | `.o+=*BOX@%&#/^` |
`n+1`  | Char for start position | `S`              |
`n+2`  | Char for last position  | `E`              |

Char list must be at least 4 chars long,
but secure char list is at least 18 chars long
and only consists of clearly distinguishable symbols.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

