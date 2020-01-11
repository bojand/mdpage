#!/usr/bin/env bash
# Installs the `hub` executable into hub/bin
set -ex
case $1 in
  ubuntu*)
    curl -LsSf https://github.com/github/hub/releases/download/v2.12.8/hub-linux-amd64-2.13.0.tgz -o hub.tgz
    mkdir hub
    tar -xzvf hub.tgz --strip=1 -C hub
    ;;
  macos*)
    curl -LsSf https://github.com/github/hub/releases/download/v2.12.8/hub-darwin-amd64-2.13.0.tgz -o hub.tgz
    mkdir hub
    tar -xzvf hub.tgz --strip=1 -C hub
    ;;
  windows*)
    curl -LsSf https://github.com/github/hub/releases/download/v2.12.8/hub-windows-amd64-2.13.0.zip -o hub.zip
    7z x hub.zip -ohub
    ;;
  *)
    echo "OS should be first parameter, was: $1"
    ;;
esac

echo "##[add-path]$PWD/hub/bin"