default: generate-protocol generate-plugins run

generate-protocol:
  cd crates/protocol && cargo run

generate-plugins:
  cargo build --target=wasm32-unknown-unknown -p starship_plugin_directory

run:
  cargo run
