FROM rust:1.93.0-alpine as builder

ARG version="dev"
ARG revision="dev"

LABEL org.opencontainers.image.authors="James Mapp Fan Club"
LABEL org.opencontainers.image.description="A Discord bot named after the infamous LLM for X, using Ollama as the LLM backend."
LABEL org.opencontainers.image.documentation="https://github.com/thejmfc/grokbot"
LABEL org.opencontainers.image.licenses="GPL-3.0"
LABEL org.opencontainers.image.revision=$revision
LABEL org.opencontainers.image.source="https://github.com/thejmfc/grokbot"
LABEL org.opencontainers.image.title="GrokBot"
LABEL org.opencontainers.image.url="https://github.com/thejmfc/grokbot"
LABEL org.opencontainers.image.version=$version

# Creates a new empty project
RUN USER=root cargo new --bin grokbot
WORKDIR /grokbot

# Copies over Cargo files
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Caches dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy over source code
COPY ./src ./src

# Build release from source code
RUN rm ./target/release/deps/grokbot*
RUN cargo build --release

FROM rust:1.93.0-alpine as runner

COPY --from=builder /grokbot/target/release/grokbot .

CMD ["./grokbot"]