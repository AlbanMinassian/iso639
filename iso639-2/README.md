# iso639_2 #

[![Build Status](https://travis-ci.org/AlbanMinassian/iso639.svg?branch=master)](https://travis-ci.org/AlbanMinassian/iso639)
[![codecov](https://codecov.io/gh/AlbanMinassian/iso639/branch/master/graph/badge.svg)](https://codecov.io/gh/AlbanMinassian/iso639)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![iso639_2 Latest Version](https://img.shields.io/crates/v/iso639_2.svg)](https://crates.io/crates/iso639_2)


## Usage ##

``Cargo.toml``

```rust
[dependencies]
iso639_2 = "0.1.12" // beware underscore
```

<script src="https://gist.github.com/AlbanMinassian/72ad2a2c052c262fd84424d28b6c1ad8.js"></script>

``src/main.rs`` ([gist](https://gist.github.com/AlbanMinassian/72ad2a2c052c262fd84424d28b6c1ad8))

```rust
extern crate iso639_2;
use iso639_2::Iso639_2;

pub fn main() {
    println!("{:?}", Iso639_2::Eng);
    assert!(Iso639_2::Fra != Iso639_2::Eng);
}
```


## Links ##

- [documentation iso639_2 (docs.rs)](https://docs.rs/iso639_2)
- [wikipedia ISO_639](https://en.wikipedia.org/wiki/ISO_639) [[en](https://en.wikipedia.org/wiki/ISO_639), [fr](https://fr.wikipedia.org/wiki/ISO_639), [de](https://de.wikipedia.org/wiki/ISO_639), [fr](https://fr.wikipedia.org/wiki/ISO_639), [es](https://es.wikipedia.org/wiki/ISO_639), [it](https://it.wikipedia.org/wiki/ISO_639)]

## License ##

Copyright © 2018, [Alban Minassian](https://github.com/AlbanMinassian)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

The Software is provided “as is”, without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders X be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the Software.
Except as contained in this notice, the name of the [Alban Minassian](https://github.com/AlbanMinassian) shall not be used in advertising or otherwise to promote the sale, use or other dealings in this Software without prior written authorization from the [Alban Minassian](https://github.com/AlbanMinassian).
