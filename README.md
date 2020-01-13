<div align="center">
	<br>
	<img src="docs/static/logo-192x192.png" alt="Logo" width="96" height="96">
	<br>
</div>

# mdpage

> Simple documentation tool

[![Build Status](https://github.com/bojand/mdpage/workflows/build/badge.svg?style=flat-square)](https://github.com/bojand/mdpage/actions?workflow=build)
[![LICENSE](https://img.shields.io/github/license/bojand/mdpage.svg?style=flat-square)](LICENSE)
[![Donate](https://img.shields.io/badge/Donate-PayPal-green.svg?style=flat-square)](https://www.paypal.me/bojandj)
[![Buy me a coffee](https://img.shields.io/badge/buy%20me-a%20coffee-orange.svg?style=flat-square)](https://www.buymeacoffee.com/bojand)

[mdPage](https://github.com/bojand/mdpage) is a minimal, opinionated, command line utility (and [Rust](https://www.rust-lang.org/) crate) for creating single-page HTML documentation from markdown files.

## Demo

The [mdPage documentation](https://bojand.github.io/mdpage) is generated using the `mdpage` CLI tool from markdown source files. There are additional examples listed in the documentation with sources available in the `docs` directory.

## Installation

There are multiple ways to install mdPage.

1. **Binaries**

   Binaries are available for download [here](ttps://github.com/bojand/mdpage/releases). Make sure to put the
   path to the binary into your `PATH`.

2. **From Crates.io**

   This requires Rust and Cargo to be installed. Once you have installed
   Rust, type the following in the terminal:

   ```
   cargo install mdpage
   ```

   This will download and compile mdPage for you, the only thing left to do is
   to add the Cargo bin directory to your `PATH`.

## Usage

`mdpage` tool takes a singe argument, the root path directory where all the markdown content is located.

```sh
$ mdpage --help
mdpage 0.1.1
Generate simple documentation

USAGE:
    mdpage [FLAGS] [OPTIONS] <path>

FLAGS:
        --full-page    Generate full page documentation
    -h, --help         Prints help information
    -V, --version      Prints version information

OPTIONS:
    -o, --output <output>        The output file
        --subtitle <subtitle>    Subtitle of the document
        --title <title>          Title of the document

ARGS:
    <path>    Path for the directory containing data
```

The result of running the tool is always an `index.html` file.

Example workflow:

```sh
$ ls ./examples/basic | sort
page1.md
page2.md
readme.md

$ mdpage ./examples/basic

$ ls ./examples/basic | sort
index.html
page1.md
page2.md
readme.md

$ open index.html
```

## License

Apache-2.0