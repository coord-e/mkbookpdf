# mkbookpdf

[![Actions Status](https://github.com/coord-e/mkbookpdf/workflows/Test%20and%20Lint/badge.svg)](https://github.com/coord-e/mkbookpdf/actions?workflow=Test+and+Lint)
[![Actions Status](https://github.com/coord-e/mkbookpdf/workflows/Release/badge.svg)](https://github.com/coord-e/mkbookpdf/actions?workflow=Release)
[![Crates.io](https://img.shields.io/crates/v/mkbookpdf)](https://crates.io/crates/mkbookpdf)
[![Crates.io](https://img.shields.io/crates/l/mkbookpdf)](https://crates.io/crates/mkbookpdf)
[![Docker Cloud Automated build](https://img.shields.io/docker/cloud/automated/coorde/mkbookpdf)](https://hub.docker.com/r/coorde/mkbookpdf)
[![Docker Cloud Build Status](https://img.shields.io/docker/cloud/build/coorde/mkbookpdf)](https://hub.docker.com/r/coorde/mkbookpdf)
[![MicroBadger Layers](https://img.shields.io/microbadger/layers/coorde/mkbookpdf)](https://microbadger.com/images/coorde/mkbookpdf)
[![MicroBadger Size](https://img.shields.io/microbadger/image-size/coorde/mkbookpdf)](https://microbadger.com/images/coorde/mkbookpdf)

`mkbookpdf` is a simple command-line utility for booklet printing.

## Usage

```shell
# prints input.pdf as a booklet
$ mkbookpdf input.pdf -p

# writes the converted PDF to output.pdf
$ mkbookpdf input.pdf -o output.pdf
```

## Installation

Platform|Download
--------|--------
Linux 64-bit|[mkbookpdf-x86_64-unknown-linux-musl](https://github.com/coord-e/mkbookpdf/releases/latest/download/mkbookpdf-x86_64-unknown-linux-musl)
macOS 64-bit|[mkbookpdf-x86_64-apple-darwin](https://github.com/coord-e/mkbookpdf/releases/latest/download/mkbookpdf-x86_64-apple-darwin)
Windows 64-bit|[mkbookpdf-x86_64-pc-windows-msvc.exe](https://github.com/coord-e/mkbookpdf/releases/latest/download/mkbookpdf-x86_64-pc-windows-msvc.exe)

Note that printing feature (`-p`, `--print`) requires `lp` utility to be installed in your system. (This means you can't use `-p` in windows)

### with Cargo

```shell
$ cargo install mkbookpdf
```

### with Docker

```shell
# place this in your shell configuration (.bashrc, .zshrc, etc...)
$ alias mkbookpdf="docker run --rm -v $(pwd)/data -v /var/run/cups:/var/run/cups coorde/mkbookpdf"
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
