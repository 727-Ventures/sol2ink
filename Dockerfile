#FROM rust
#
#RUN apt-get update && \
#    apt-get install libclang-dev -y && \
#
#RUN curl -sSf https://sh.rustup.rs/ | sh -s -- --default-toolchain nightly -y
#
#RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
#RUN rustup target add wasm32-unknown-unknown
#
#RUN cargo install cargo-dylint dylint-link