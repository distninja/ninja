#!/bin/bash

# Enable android soong: ANDROID_SOONG (--android-soong)
# Enable function trace: FUNCTION_TRACE (--function-trace)
# Enable lmdb store: LMDB_STORE (--lmdb-store)

if grep -qi microsoft /proc/version; then
    echo "Running on Windows Subsystem for Linux"
    cmake -G "Ninja" -DCMAKE_BUILD_TYPE=Release -DANDROID_SOONG=ON -DLMDB_STORE=ON -B release-build
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    echo "Running on Windows"
    cmake -G "Visual Studio 17 2022" -DCMAKE_BUILD_TYPE=Release -DANDROID_SOONG=ON -DLMDB_STORE=ON -B release-build
elif [[ -f /etc/os-release ]] && grep -qi ubuntu /etc/os-release; then
    echo "Running on Ubuntu"
    cmake -G "Ninja" -DCMAKE_BUILD_TYPE=Release -DANDROID_SOONG=ON -DLMDB_STORE=ON -B release-build
elif [[ -f /etc/os-release ]] && grep -qi rocky /etc/os-release; then
    echo "Running on Rocky"
    cmake -G "Ninja" -DCMAKE_BUILD_TYPE=Release -DANDROID_SOONG=ON -DLMDB_STORE=ON -B release-build
else
    echo "Unknown OS"
fi

cmake --build release-build --parallel --config Release
