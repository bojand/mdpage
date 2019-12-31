## mdpage

[mdPage](https://github.com/bojand/mdpage) is a simple, opinionated, command line tool (and a Rust crate) to create single-page HTML documentation from markdown files. At it's simplest basic form of usage, it takes no input and should generate practical documentation from all markdown files in a folder. 

Indeed this very documentation is generated using `mdpage` tool with some minor configuration and serves as an example of generated output.

The basic goal is simply to generate a self-contained single page HTML file for all documentation in a folder (and first-level subfolders). The tool has _some_ opinions on structure and patterns used, and has limited configuration and customization options. It lets you focus on creating the content and not waste time on style or format of the output.