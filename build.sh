#!/bin/bash



wasm-pack build --release --target web

cp index.html pkg/index.html
