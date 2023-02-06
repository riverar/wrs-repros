#!/bin/bash
pushd .metadata
gcc -c -o sample.o sample.cpp
ar -rcs libsamplestatic.a sample.o
rm sample.o
mv libsamplestatic.a ../.windows
popd