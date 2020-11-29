## Rust Util


Config `Cargo.toml`:
```
[dependencies]
rust_util = "0.6"

--OR--

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

<br>

ENV `LOGGER_LEVEL` can be:
- `debug` or `*`
- `info` or `?` -- default
- `ok` or `#`
- `warn` or `!`
- `error` or `^`


## Update Log

* Nov 28, 2020 v0.6.19
    * add util_git
* Nov 28, 2020 v0.6.18
    * add util_term
* Jun 21, 2020 v0.3.0
    * add struct `JoinFilesReader`

