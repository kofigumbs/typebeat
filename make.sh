#!/usr/bin/env bash

set -ex

mkdir -p build
git submodule update --init --recursive
faust -os -cn Effects -o build/Effects.h audio/Effects.dsp
g++ desktop/main.cpp -std=c++17 -I build -I `faust --includedir` -ldl -lm -lpthread -framework WebKit -o build/groovebox
