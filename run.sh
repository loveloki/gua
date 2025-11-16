#!/bin/bash

dist build && RUST_BACKTRACE=1 ./target/distrib/*/gua
