#!/usr/bin/env bash

cargo build && cp -f ../../target/debug/libxfade.so xfade~.pd_linux && pd test.pd
