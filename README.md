<div align="center">
<h1>bishop.rs</h1>

<a href="https://travis-ci.org/AbsurdlySuspicious/bishop.rs">
<img src="https://travis-ci.org/AbsurdlySuspicious/bishop.rs.svg?branch=master">
</a>
<a href="https://crates.io/crates/bishop">
<img src="https://img.shields.io/crates/v/bishop">
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

Library for visualizing data using The Drunken Bishop algorithm implemented in Rust

Drunken Bishop is the algorithm used in OpenSSH's `ssh-keygen` for visualising generated keys 

**CLI app:**

[![GitHub](https://img.shields.io/badge/-GitHub-grey?logo=github)](https://github.com/AbsurdlySuspicious/bishop.rs/tree/master/bishop-cli)
[![crates.io](https://img.shields.io/badge/-crates.io-orange?logo=rust)](https://crates.io/crates/bishop-cli)

`bishop-cli` (`bishop`) - CLI app for visualising hex or binary data

---

Reference used for this implementation:
http://www.dirk-loss.de/sshvis/drunken_bishop.pdf

## Example

`Cargo.toml`:

```toml
[dependencies]
bishop = "x.x.x"
```

Use current latest version:
![Latest version](https://img.shields.io/crates/v/bishop?label=&color=grey) 

`main.rs`:

```rust
use bishop::bishop as bs;

use std::io::{Read, BufReader};
use std::fs::File;

fn main() {
    let cfg = bs::Options::default();

    // from file to String
    let file = File::open("some_file").unwrap();
    let art = bs::art_str(&mut BufReader::new(file).bytes(), &cfg).unwrap();
    println!("{}", art);

    // from vec to String
    let src = vec![1u8, 3, 3, 7];
    let art = bs::art_str(&src, &cfg).unwrap();
    println!("{}", art);

    // from slice to stdout
    let src = [1u8, 2, 3, 4, 5];
    bs::art_print(src.as_ref(), &cfg, |p| println!("{}", p)).unwrap();
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

