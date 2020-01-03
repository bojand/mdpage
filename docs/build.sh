#!/bin/bash

./target/debug/mdpage ./docs/examples/basic
./target/debug/mdpage ./docs/examples/sections
./target/debug/mdpage ./docs/examples/full_page --full-page --title "Full Page" --subtitle "Full page example"
./target/debug/mdpage ./docs/examples/config
./target/debug/mdpage ./docs/