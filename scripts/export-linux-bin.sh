#!/usr/bin/env bash

name=$1
out=$2
mkdir -p "$out"
cp "./result/bin/$name" "$out/$name-x86_64-linux"
chmod 777 "$out/$name-x86_64-linux"
patchelf --set-interpreter /lib64/ld-linux-x86-64.so.2 --set-rpath /lib/x86_64-linux-gnu "$out/$name-x86_64-linux"
