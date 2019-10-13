# mkbookpdf

`mkbookpdf` is a simple command-line utility for booklet printing.

## Usage

```shell
# print input.pdf as a booklet 
$ mkbookpdf input.pdf -p

# save converted PDF as output.pdf
$ mkbookpdf input.pdf -o output.pdf
```

## Installation

Platform|Download
Linux 64-bit|[mkbookpdf-x86_64-unknown-linux-musl](https://github.com/coord-e/mkbookpdf/releases/latest/download/mkbookpdf-x86_64-unknown-linux-musl)
macOS 64-bit|[mkbookpdf-x86_64-apple-darwin](https://github.com/coord-e/mkbookpdf/releases/latest/download/mkbookpdf-x86_64-apple-darwin)
Windows 64-bit|[mkbookpdf-x86_64-pc-windows-msvc](https://github.com/coord-e/mkbookpdf/releases/latest/download/mkbookpdf-x86_64-pc-windows-msvc)

### with Cargo

```shell
$ cargo install mkbookpdf
```

### with Docker

```shell
# place this in your shell configuration (.bashrc, .zshrc, etc...)
$ alias mkbookpdf="docker run --rm -v $(pwd)/data -v /var/run/cups:/var/run/cups coorde/mkbookpdf"
```
