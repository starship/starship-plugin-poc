default: generate-protocol generate-plugins run

generate-protocol:
  cd crates/protocol && cargo run

generate-plugins:
  cd plugins/directory && cargo build

run:
  cargo run -p starship_daemon

clean:
  rm -rf target \
  && rm -rf crates/starship_plugin \
  && rm -rf plugins/directory/target \
  && rm -rf crates/daemon/src/plugin_runtime/bindings.rs \
  && rm -rf crates/daemon/src/plugin_runtime/types.rs
