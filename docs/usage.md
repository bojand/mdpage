## Usage

`mdpage` takes a singe argument, the root path directory where all the content is located.

```sh
$ mdpage --help
mdpage 0.1.0
Generate simple documentation

USAGE:
    mdpage <path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <path>    Path for the directory contianing data
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

