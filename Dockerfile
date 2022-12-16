ARG RUST_VERSION="1.66.0"
ARG DEBIAN_VERSION="bullseye"


FROM rust:${RUST_VERSION}-${DEBIAN_VERSION} as builder

WORKDIR /code
COPY . /code

SHELL ["/bin/bash", "-c", "-o", "pipefail"]
RUN cargo build --release


# hadolint ignore=DL3007
FROM gcr.io/distroless/cc-debian11:latest

LABEL org.opencontainers.image.authors="Vlad Vasiliu"

EXPOSE 9925
ENV STATUSPAGE_EXPORTER_LISTEN="0.0.0.0:9925"

COPY --from=builder /code/target/release/statuspage-exporter /

CMD ["/statuspage-exporter"]
