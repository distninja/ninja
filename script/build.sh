#!/bin/bash

# Enable android soong: ANDROID_SOONG (--android-soong)
# Enable function trace: FUNCTION_TRACE (--function-trace)
cmake -GNinja -DCMAKE_BUILD_TYPE=Release -DANDROID_SOONG=ON -B release-build
cmake --build release-build --parallel --config Release
strip release-build/ninja
