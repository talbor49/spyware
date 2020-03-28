#!/bin/bash
cargo build --release && cargo strip
ls -lah target/release/spyware
