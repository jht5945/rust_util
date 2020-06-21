## Rust Util


Config `Cargo.toml`:
```
[dependencies]
rust_util = { git = "https://github.com/jht5945/rust_util" }
```

Use in `*.rs`:
```
extern crate rust_util;

...

use rust_util::*;
```

Update git crate:
```
$ cargo update
```


## Update Log

* Jun 21, 2020 v0.3.0
    * add struct `JoinFilesReader`

