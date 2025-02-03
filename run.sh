#!/bin/sh

ls Cargo.toml | ENV_NUM_BYTES_TO_SKIP=0 ./rs-cut-lines-ascii
ls Cargo.toml | ENV_NUM_BYTES_TO_SKIP=1 ./rs-cut-lines-ascii
ls Cargo.toml | ENV_NUM_BYTES_TO_SKIP=2 ./rs-cut-lines-ascii
