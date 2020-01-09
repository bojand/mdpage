## mdpage

[![Build Status](https://github.com/bojand/mdpage/workflows/build/badge.svg?style=flat-square)](https://github.com/bojand/mdpage/actions?workflow=build)
[![LICENSE](https://img.shields.io/github/license/bojand/mdpage.svg?style=flat-square)](LICENSE)
[![Donate](https://img.shields.io/badge/Donate-PayPal-green.svg?style=flat-square)](https://www.paypal.me/bojandj)
[![Buy me a coffee](https://img.shields.io/badge/buy%20me-a%20coffee-orange.svg?style=flat-square)](https://www.buymeacoffee.com/bojand)

[mdPage](https://github.com/bojand/mdpage) is a minimal, opinionated, command line utility (and [Rust](https://www.rust-lang.org/) crate) for creating single-page HTML documentation from markdown files. At it's simplest basic form of usage, it takes no input and should generate practical documentation from all markdown files in a folder. 

Indeed this very documentation is generated using `mdpage` tool with some minor configuration and serves as an example of generated output.

The basic goal is simply to generate a self-contained single page HTML file for all documentation in a folder (and first-level subfolders). The tool has _some_ opinions on structure and patterns used, and has limited configuration and customization options. It purposefully lacks in features, to let you focus on creating the content and not waste time on style or format of the output.