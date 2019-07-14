#!/usr/bin/env bash

cargo build && cp -f ../../target/debug/libhelloworld.so helloworld.pd_linux && pd test.pd
