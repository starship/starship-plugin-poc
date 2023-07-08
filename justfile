default: generate-protocol generate-plugins run

generate-protocol:
  cd crates/protocol && cargo run

generate-plugins:
  cd plugins/directory && cargo build

run:
  cargo run -p starship_daemon
