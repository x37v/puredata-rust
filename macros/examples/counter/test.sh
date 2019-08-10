#!/usr/bin/env bash

cargo build && cp -f ./target/debug/libcounter.so counter.pd_linux && pd test.pd
