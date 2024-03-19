ARG RUST_VERSION="1.76.0"
ARG DEBIAN_VERSION="buster"


FROM rust:${RUST_VERSION}-${DEBIAN_VERSION} as builder

RUN cargo install cargo-auditable

WORKDIR /code
COPY . /code

RUN cargo --config net.git-fetch-with-cli=true auditable build --release


# hadolint ignore=DL3007
FROM gcr.io/distroless/cc-debian12:latest

LABEL org.opencontainers.image.authors="Vlad Vasiliu"

EXPOSE 9925
ENV STATUSPAGE_EXPORTER_LISTEN="0.0.0.0:9925"

COPY --from=builder /code/target/release/statuspage-exporter /

CMD ["/statuspage-exporter"]
