#!/bin/bash

# 测试、编译、运行
cargo test && dist build && RUST_BACKTRACE=1 ./target/distrib/*/gua
