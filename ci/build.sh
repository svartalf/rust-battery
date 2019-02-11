#!/bin/bash

set -ex

if [[ $TRAVIS_OS_NAME == 'osx' ]]; then
    CARGO_BIN=cargo
else
    CARGO_BIN=cross
fi

${CARGO_BIN} build --target="${TARGET}"
${CARGO_BIN} test --target="${TARGET}"
