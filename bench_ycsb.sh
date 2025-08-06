#!/bin/bash

set -e

hyperfine \
  --prepare "cargo build --release --target-dir target" \
  --parameter-list spec a,b,c,d,e,f \
  "target/release/tectonic-cli generate -w ./ycsb-specs/{spec}.spec.json" \
  --export-json ycsb.json
