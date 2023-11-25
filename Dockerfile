# Stage 1: Build the dependencies
FROM rust as builder

WORKDIR /myapp

COPY ./Cargo.toml ./Cargo.lock ./

RUN mkdir src && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release
RUN rm -rf src/*


# Stage 2: Build the actual application
COPY ./src ./src
RUN touch src/*.rs && cargo build --release


# Stage 3: Create the final image
FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /myapp/target/release/simple-rss-creator simple-rss-creator
COPY static-files /app/static-files
USER 1000:1000
EXPOSE 8080
ENTRYPOINT ["/app/simple-rss-creator"]
