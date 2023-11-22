# Build Stage
FROM rust:1.65.0-slim as builder

COPY ../ ../
# Build empty app with downloaded dependencies to produce a stable image layer for next build
RUN apt-get install -y cmake && cargo build --release

# Cmd
CMD ["../target/release/knowmadmood-saco-rust-tokio"]