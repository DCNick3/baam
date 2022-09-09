# syntax = docker/dockerfile:1.2

FROM clux/muslrust:stable as chef
RUN cargo install cargo-chef --locked

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
COPY --from=planner /volume/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN --mount=type=cache,target=/root/.cargo/registry --mount=type=cache,target=/volume/target \
    cargo chef cook --profile ship --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Build application
COPY . .
RUN --mount=type=cache,target=/root/.cargo/registry --mount=type=cache,target=/volume/target \
    cargo b --profile ship --target x86_64-unknown-linux-musl && \
    cp target/x86_64-unknown-linux-musl/ship/baam baam

FROM gcr.io/distroless/static

LABEL org.opencontainers.image.source https://github.com/DCNick3/baam
EXPOSE 8080

COPY --from=build /volume/baam /baam

CMD ["/baam"]

