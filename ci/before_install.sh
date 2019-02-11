#!/bin/bash

set -ex

if [[ $TRAVIS_OS_NAME == 'osx' ]]; then
    echo "OSX is using plain cargo binary"
else
    cargo install cross
    rustup target add ${TARGET}
fi
