## Examples

All examples can be viewed on [GitHub](https://github.com/bojand/mdpage).

### Basic

[Basic example](/examples/basic)

No custom configuration.

Structure:

```
./docs/examples/basic
├── page1.md
├── page2.md
└── readme.md
```

Command:

```sh
$ mdpage ./docs/examples/basic
```

### Sections 

No custom configuration. Demonstrates how different directories are automatically treated as different sections of content.

[Sections example](/examples/sections)

Structure:

```
./docs/examples/sections
├── index.md
├── guide
│  ├── install.md
│  └── readme.md
└── reference
   ├── api.md
   └── config.md
```

Command:

```sh
$ mdpage ./docs/examples/sections
```

### Full page

[Full page example](/examples/full_page)

Using command line to specify custom title, subtitle, and full page options.
Automatically injects footer because we have `footer.md`.

Structure:

```
./docs/examples/full_page
├── index.md
├── code.md
├── footer.md
├── guide
│  ├── install.md
│  └── readme.md
└── reference
   ├── api.md
   └── config.md
```

Command:

```
$ mdpage ./docs/examples/full_page --full-page --title "Full Page" --subtitle "Full page example"
```

### Config 

Customized build using `mdpage.json` file. Specifies script and style links to add [highlight.js](https://highlightjs.org/) support. Specifies content to control order of menu items and sections, and to add external links.

[Config example](/examples/config)

Structure:

```
./docs/examples/config
├── mdpage.json
├── page1.md
└── readme.md
```

Command:

```
$ mdpage ./docs/examples/config
```

### mdPage

This documentation is created using mdPage using a header and footer, and a config file to customize content.

Structure:

```
./docs
├── config.md
├── examples.md
├── footer.md
├── header.md
├── index.html
├── index.md
├── install.md
├── usage.md
├── TODO.md
├── mdpage.json
├── static
│  ├── android-chrome-192x192.png
│  └── favicon.ico
├── examples
   ├── basic
   ├── config
   ├── full_page
   └── sections
```

Command:

```
$ mdpage ./docs
```

The markdown content in `examples` subdirectories is automatically ignored since it's too deep. `mdpage` only looks at content in immediate subdirectories.
