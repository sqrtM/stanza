FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

# Create a dummy main file to build dependencies
# Going to probably just replace this with cargo chef
#RUN mkdir src && echo "fn main() {}" > src/main.rs
#RUN cargo build --release
#RUN rm -f src/main.rs

COPY src ./src
RUN cargo build --release
FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/release/stanza /usr/local/bin/stanza
EXPOSE 8000
CMD ["stanza"]
