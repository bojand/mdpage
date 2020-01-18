#!/bin/bash

/.cargo/bin/mdpage ./docs/examples/basic
/.cargo/bin/mdpage ./docs/examples/sections
/.cargo/bin/mdpage ./docs/examples/full_page --full-page --title "Full Page" --subtitle "Full page example"
/.cargo/bin/mdpage ./docs/examples/config
/.cargo/bin/mdpage ./docs/examples/single_index --title "Single page" --subtitle "Single main page"
/.cargo/bin/mdpage ./docs/examples/single_page --title "Single page" --subtitle "Single content page"
/.cargo/bin/mdpage ./docs/examples/config_dir
/.cargo/bin/mdpage ./docs
