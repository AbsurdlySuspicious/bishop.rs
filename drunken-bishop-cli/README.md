<div align="center">
<h1>bishop-cli</h1>

<a href="https://travis-ci.org/AbsurdlySuspicious/bishop.rs">
<img src="https://travis-ci.org/AbsurdlySuspicious/bishop.rs.svg?branch=master">
</a>
<a href="https://crates.io/crates/bishop-cli">
<img src="https://img.shields.io/crates/v/bishop-cli">
</a>

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

CLI app for visualising hex or binary data using The Drunken Bishop algorithm implemented in Rust

Drunken Bishop is the algorithm used in OpenSSH's `ssh-keygen` for visualising generated keys 

**Rust library:**

[![GitHub](https://img.shields.io/badge/-GitHub-grey?logo=github)](https://github.com/AbsurdlySuspicious/bishop.rs)
[![crates.io](https://img.shields.io/badge/-crates.io-orange?logo=rust)](https://crates.io/crates/bishop)

`drunken-bishop` - Library for visualizing any `&[u8]` or `Read`

<!--
**AUR package:**

![AUR](https://img.shields.io/aur/version/bishop?label=aur%20/%20bishop)
-->

---

Reference used for this implementation:
http://www.dirk-loss.de/sshvis/drunken_bishop.pdf

## Usage

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
```

## Examples

### Passing hex as argument

`$ bishop $(echo foobar | sha256sum | cut -d' ' -f1)`

```
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
```

### Using `-i`

`$ cat foobar.bin | bishop -i -`

or

`$ bishop -i foobar.bin`

```
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

Note that with `-i` option data is treated as **binary**, **not hex** 

## Help

### Char list

Char list is a string each char of which is treated as:

_indexing starts from 1_

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

