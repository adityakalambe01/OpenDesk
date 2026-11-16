# Run the Rust workspace without installing Rust on the host.
FROM rust:1.82-bookworm

WORKDIR /workspace

COPY Cargo.toml Cargo.lock ./
COPY core ./core

# Fetch dependencies before adding the remaining repository files.
RUN cargo fetch --locked

COPY . .

CMD ["cargo", "test", "--workspace", "--locked"]
