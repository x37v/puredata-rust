#!/usr/bin/env bash

cargo build && cp -f ../../target/debug/libpan.so pan~.pd_linux && pd test.pd
