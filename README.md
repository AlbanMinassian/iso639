# iso639 #

[![Build Status](https://travis-ci.org/AlbanMinassian/iso639.svg?branch=master)](https://travis-ci.org/AlbanMinassian/iso639)
[![codecov](https://codecov.io/gh/AlbanMinassian/iso639/branch/master/graph/badge.svg)](https://codecov.io/gh/AlbanMinassian/iso639)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)


## Example

``Cargo.toml`` :

```rust
[dependencies]
iso639-1 = "0.1.12"
```

``src/main.rs`` :

```rust
extern crate iso639_1;
use iso639_1::{Iso639_1, get_enum, get_code_iso639_3};

pub fn main() {
    println!("{:?}", Iso639_1::En);
    println!("{:?}", get_enum("it"));
    println!("{}", get_code_iso639_3("fr"));
}
```
## Iso639-1 ##

See full [README.md](./iso639-1/README.md)

[![iso639-1 Latest Version](https://img.shields.io/crates/v/iso639-1.svg)](https://crates.io/crates/iso639-1)

## Iso639-2 ##

See full [README.md](./iso639-2/README.md)

[![iso639_2 Latest Version](https://img.shields.io/crates/v/iso639_2.svg)](https://crates.io/crates/iso639_2)



## Links ##

- [documentation iso639-1 (docs.rs)](https://docs.rs/iso639-1)
- [documentation iso639_2 (docs.rs)](https://docs.rs/iso639_2)
- [wikipedia ISO_639](https://en.wikipedia.org/wiki/ISO_639) [[en](https://en.wikipedia.org/wiki/ISO_639), [fr](https://fr.wikipedia.org/wiki/ISO_639), [de](https://de.wikipedia.org/wiki/ISO_639), [fr](https://fr.wikipedia.org/wiki/ISO_639), [es](https://es.wikipedia.org/wiki/ISO_639), [it](https://it.wikipedia.org/wiki/ISO_639)]

## License ##

Copyright © 2018, [Alban Minassian](https://github.com/AlbanMinassian)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

The Software is provided “as is”, without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders X be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the Software.
Except as contained in this notice, the name of the [Alban Minassian](https://github.com/AlbanMinassian) shall not be used in advertising or otherwise to promote the sale, use or other dealings in this Software without prior written authorization from the [Alban Minassian](https://github.com/AlbanMinassian).
