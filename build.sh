#!/bin/bash

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

# Build the project
cargo build --release

# Create a public directory and copy the binary there
mkdir -p public
cp target/release/bs public/bs

# Create a simple index.html that will run the binary
echo '<html><body><script>fetch("/bs").then(response => response.text()).then(text => document.body.innerHTML = text);</script></body></html>' > public/index.html