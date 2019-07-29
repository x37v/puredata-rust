#!/usr/bin/env bash

cargo build && cp -f ../../target/debug/libcomplex_counter.so counter.pd_linux && pd test.pd
