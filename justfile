default: generate-protocol generate-plugins run

generate-protocol:
  cargo run -p starship_plugin_protocol

generate-plugins:
  cargo build -p starship_plugin_directory --target=wasm32-unknown-unknown

run:
  cargo run -p starship_daemon

clean:
  rm -rf target \
  && rm -rf crates/starship_plugin \
  && rm -rf plugins/directory/target \
  && rm -rf crates/daemon/src/plugin_runtime/bindings.rs \
  && rm -rf crates/daemon/src/plugin_runtime/types.rs
