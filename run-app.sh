cargo build --all \
  && RUST_BACKTRACE=1 cargo watch -i "*/engine/**" -i -x "run"
