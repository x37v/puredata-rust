#!/usr/bin/env bash

cargo build && cp -f ./target/debug/libhelloall.so helloall.pd_linux && pd test.pd
