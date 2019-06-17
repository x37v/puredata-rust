#!/usr/bin/env bash

cargo build && cp -f target/debug/libhellodsp.so hellodsp~.pd_linux && pd test.pd
