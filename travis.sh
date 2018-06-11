#!/usr/bin/env bash

set -e

cargo test $FEATURES
cargo test $FEATURES --no-default-features
