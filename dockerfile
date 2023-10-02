FROM rust:latest as builder

# Prepare the build environment
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /build

# Build the project
COPY Cargo.toml Cargo.lock ./
COPY migrations ./migrations
COPY .sqlx ./.sqlx
COPY src ./src
RUN cargo build --target x86_64-unknown-linux-musl --release

# Build the final image
FROM scratch

WORKDIR /app

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/birthday_bot ./birthday_bot
COPY ./migrations ./migrations

CMD ["./birthday_bot"]