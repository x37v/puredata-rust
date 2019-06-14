#!/usr/bin/env bash

cargo build && mv target/debug/libhellodsp.so hellodsp~.pd_linux && pd test.pd
