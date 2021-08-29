#!/bin/bash

set -e

pushd solaredge-reqwest || die
cargo readme --template=../README.tpl --output=README.md
popd

pushd solaredge || die
cargo readme --template=../README.tpl --output=README.md
cp -v README.md ../
popd
