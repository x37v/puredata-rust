#!/usr/bin/env bash

cargo build && mv ../../target/debug/libhelloworld.so helloworld.pd_linux && pd test.pd
