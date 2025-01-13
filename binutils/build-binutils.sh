#!/usr/bin/env bash

clean_and_rebuild=false

if [ "$clean_and_rebuild" = true ] ; then
	rm -rf ./binutils-build
	mkdir ./binutils-build
fi

cd ./binutils-build \
	&& ../binutils-gdb/configure \
	&& make

