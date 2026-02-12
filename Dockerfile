FROM rust:1.93.0 AS builder

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

WORKDIR /build
COPY . /build
RUN cargo build --release

FROM debian AS runner

WORKDIR /app
COPY --from=builder /build/target/release/grokbot ./
RUN chmod +x /app/grokbot

CMD ["/app/grokbot"]