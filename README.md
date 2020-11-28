## Rust Util


Config `Cargo.toml`:
```
[dependencies]
rust_util = { git = "https://github.com/jht5945/rust_util" }
```

Use in `*.rs`:
```
#[macro_use]
extern crate rust_util;

...

use rust_util::*;
```

Update git crate:
```
$ cargo update
```

Run example:
```
$ cargo run --example log
```


## Update Log

* Nov 28, 2020 v0.6.18
    * add util_term
* Jun 21, 2020 v0.3.0
    * add struct `JoinFilesReader`

