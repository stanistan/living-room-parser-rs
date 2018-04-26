# living-room-parser

[![Build Status](https://travis-ci.org/stanistan/living-room-parser-rs.svg?branch=master)](https://travis-ci.org/stanistan/living-room-parser-rs)

A Rust/wasm implementation of [living-room/parser-js][1].

## Requirements

In order to work on the core of this parser, you should be able to use stable rust,
however when building out the JS/WASM, portions of the library, you also need:

1. [stdweb][2]
2. [npm][3] (obvs)
3. make

## Building the JS lib

After iterating/testing the rust lib and making sure it functions correctly...

```sh
# You will only need to do this ONCE!
cargo install -f cargo-web
```

```sh
cd js

# will delete the already existing files
make clean

# runs `cargo web build` and copies over the `.js` and `.wasm`
# from the target/... directory
make

# will run node tests
make test
```

[1]: https://github.com/living-room/parser-js
[2]: https://github.com/koute/stdweb/
[3]: npmjs.com/
