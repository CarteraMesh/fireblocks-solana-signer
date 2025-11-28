#!/bin/bash

set -e
set -o pipefail

PACKAGES=$(cargo metadata --format-version 1 --no-deps | jq -r '.packages[] | select(.targets[] | .kind[] == "lib" or .kind[] == "bin") | .manifest_path')
if ! cargo machete "$@"; then
    echo "Detected unused dependencies" > /dev/stderr
    exit 1
fi
