FROM rust:1.86.0 AS build
WORKDIR /usr/src/myapp
COPY Cargo.toml Cargo.lock .
RUN mkdir src/ && echo "fn main() {println!(\"broken build\")}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/patch_cd*
COPY . .
RUN cargo build --release
RUN cargo install --path .

FROM ubuntu:latest
COPY --from=build /usr/local/cargo/bin/patch-cd /usr/local/bin/patch-cd
ENTRYPOINT ["/usr/local/bin/patch-cd"]
