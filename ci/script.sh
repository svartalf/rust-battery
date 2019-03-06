#!/usr/bin/env bash

set -ex

cross build --target=${TARGET}

if [ "${TARGET}" = "i686-unknown-freebsd" ] || [ "${TARGET}" = "x86_64-unknown-freebsd" ]; then
    echo "'cross test' command is not available for '${TARGET}' target"
else
    cross test --target=${TARGET}
fi

cross clippy --target=${TARGET} -- -D warnings
