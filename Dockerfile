# syntax = docker/dockerfile:1.2

FROM clux/muslrust:stable as chef
RUN cargo install cargo-chef --locked

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build

ENV NVM_VERSION=0.39.1
ENV NODE_VERSION=18.9.0
RUN apt install -y curl
RUN unset PREFIX && \
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v${NVM_VERSION}/install.sh | bash

RUN unset PREFIX && \
    . $HOME/.nvm/nvm.sh && \
    nvm install ${NODE_VERSION} && \
    nvm use v${NODE_VERSION}

ENV PATH="/root/.nvm/versions/node/v${NODE_VERSION}/bin/:${PATH}"

RUN node --version

COPY --from=planner /volume/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN \
    cargo chef cook --profile ship --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Build application
COPY . .
RUN \
    cargo b --profile ship --target x86_64-unknown-linux-musl && \
    cp target/x86_64-unknown-linux-musl/ship/baam baam

FROM gcr.io/distroless/static

LABEL org.opencontainers.image.source https://github.com/DCNick3/baam
EXPOSE 8080

COPY --from=build /volume/baam /baam

CMD ["/baam"]

