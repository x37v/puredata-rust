#!/usr/bin/env bash

cargo build && cp -f ../../target/debug/libhellodsp.so hellodsp~.pd_linux && pd test.pd
#cargo build && cp -f target/debug/libhellodsp.dylib hellodsp~.pd_darwin && open test.pd
