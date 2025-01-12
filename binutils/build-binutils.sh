#!/usr/bin/env bash

rm -rf ./binutils-build
mkdir ./binutils-build

cd ./binutils-build \
	&& ../binutils-gdb/configure \
	&& make
