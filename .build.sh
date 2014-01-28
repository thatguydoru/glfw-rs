#!/bin/sh

mkdir build
pushd build 1>&2
cmake .. 1>&2
make lib 1>&2 # cargo-lite wants stdout
echo "cargo-lite: artifacts"
find build/lib -type f
popd 1>&2