#!/bin/bash

# Simple script to run all the tests and all the examples
# with all the supported features sequentially.

# Run tests & examples for tokio
echo "-----------------------"
echo "Running tests for tokio"
echo "-----------------------"
cargo test --all-targets --no-default-features --features runtime-tokio || exit 1

# Run tests & examples for async-std
echo "---------------------------"
echo "Running tests for async std"
echo "---------------------------"
cargo test --all-targets --no-default-features --features runtime-async-std || exit 1
