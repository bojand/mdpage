## Installation

<br>

### Install from binaries

Precompiled binaries are provided for major platforms on a best-effort basis.
Visit [the releases page](https://github.com/bojand/mdpage/releases)
to download the appropriate version for your platform.

<br>

### Install from source

mdPage can also be installed from source

#### Pre-requisite

mdPage is written in [Rust](https://www.rust-lang.org/) and therefore needs
to be compiled with Cargo. If you haven't already installed Rust, please go
ahead and [install it](https://www.rust-lang.org/tools/install) now.

#### Install Crates.io version

Installing mdPage is relatively easy if you already have Rust and Cargo
installed. You just have to type this snippet in your terminal:

```bash
cargo install mdpage
```

This will fetch the source code for the latest release from
[Crates.io](https://crates.io/) and compile it. You will have to add Cargo's
`bin` directory to your `PATH`.

Run `mdpage --help` in your terminal to verify if it works. Congratulations, you
have installed mdPage!