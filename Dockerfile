#syntax=syntax=docker/dockerfile:1.3-labs
# Build rust
FROM docker.io/rustlang/rust:nightly-slim as neko
WORKDIR /build
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.toml.bak
# Cache dependencies by making a fake project
RUN <<EOF
echo '[package]\nname = "init"\nversion = "0.0.0"\nedition = "2021"\n[dependencies]\n' > Cargo.toml
mkdir src && touch src/lib.rs
cat Cargo.toml.bak | sed -n '/# external crates/,$p' >> Cargo.toml
cargo build --release
rm -rf src Cargo.toml.bak Cargo.toml
EOF
COPY . .
RUN cargo build --release

# Runtime
FROM debian:11-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates libssl1.1 openssl && rm -rf /var/lib/apt/lists/*
COPY --from=neko /build/target/release/neko /app/neko
CMD ["./neko"]
