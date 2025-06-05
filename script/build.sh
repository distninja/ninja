#!/bin/bash

# Enable android soong: ANDROID_SOONG (--android-soong)
# Enable function trace: FUNCTION_TRACE (--function-trace)
# Enable lmdb store: LMDB_STORE (--lmdb-store)

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "Running on Linux"
    cmake -G "Ninja" -DCMAKE_BUILD_TYPE=Release -DANDROID_SOONG=ON -DLMDB_STORE=ON -B release-build
    cmake --build release-build --parallel --config Release
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    echo "Running on Windows (MinGW/MSYS/Cygwin)"
    CC=gcc CXX=g++ cmake -G "MinGW Makefiles" -DCMAKE_BUILD_TYPE=Release -DANDROID_SOONG=ON -DLMDB_STORE=ON -B release-build
    pushd release-build; mingw32-make -j4; popd
fi
