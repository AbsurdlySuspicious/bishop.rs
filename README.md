<div align="center">
<h1>drunken-bishop.rs</h1>

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

Implementaion of The Drunken Bishop visual fingerprint algorithm
(that one used for so-called *randomarts* in **ssh-keygen**) in Rust.
This package provides library crate and command-line application for visualising any binary or hex-formatted data.

Reference used for this implementation:
http://www.dirk-loss.de/sshvis/drunken_bishop.pdf

## Examples

### Using as command-line utility
`TODO`

### Using as library in my project
`TODO`

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

