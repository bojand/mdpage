#!/bin/bash

/home/runner/.cargo/bin/mdpage ./docs/examples/basic
/home/runner/.cargo/bin/mdpage ./docs/examples/sections
/home/runner/.cargo/bin/mdpage ./docs/examples/full_page --full-page --title "Full Page" --subtitle "Full page example"
/home/runner/.cargo/bin/mdpage ./docs/examples/config
/home/runner/.cargo/bin/mdpage ./docs/examples/single_index --title "Single page" --subtitle "Single main page"
/home/runner/.cargo/bin/mdpage ./docs/examples/single_page --title "Single page" --subtitle "Single content page"
/home/runner/.cargo/bin/mdpage ./docs/examples/config_dir
/home/runner/.cargo/bin/mdpage ./docs
