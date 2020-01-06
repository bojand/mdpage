## Usage

`mdpage` tool takes a singe argument, the root path directory where all the content is located.

```sh
$ mdpage --help
mdpage 0.1.0
Generate simple documentation

USAGE:
    mdpage [FLAGS] [OPTIONS] <path>

FLAGS:
        --full-page    Generate full page documentation
    -h, --help         Prints help information
    -V, --version      Prints version information

OPTIONS:
        --subtitle <subtitle>    Subtitle
        --title <title>          Title

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
```

This is exacly how the [basic](/examples/basic) was generated!

The title of the generated documents is automatically inferred from the `readme.md` (or `index.md`) document's heading, or, if not possible, from the directory name. Similarly the page links in the menu are inferred from the file headings, or, if not possible, from the file name.

mdPage expects **all** contents to be in the immediate directory as root of the content and immediate subdirectories. It makes certain assumptions about structure and meaning. The content in subdirectories is separated out and used as different "sections" in the menu. Similar to the rules stated above, the section title is inferred from heading `readme.md` (or `index.md`) or, if not possible, from the directory name.

Take a look at the [sections example](/examples/sections) for an idea of how that works and what the result looks like.

We can customize some things via command line options such as title, subtitle, and whether to generate a single full page document (no "JavaScript" pagination).

```sh
$ mdpage ./docs/examples/full_page --full-page --title "Full Page" --subtitle "Full page example"
```

See the resulting [full page example](/examples/full_page).

All of this and more can be controlled via a configuration file, but the basic idea is that the tool in most cases should _just work_.

<br>

### Debug logging

mdPage uses [env_logger crate](https://crates.io/crates/env_logger), which can be used to enable logging.

```sh
RUST_LOG=info mdpage .
```

or for `debug` level logging:

```sh
RUST_LOG=debug mdpage .
```