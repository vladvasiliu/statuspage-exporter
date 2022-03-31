ARG RUST_VERSION="1.59.0"
ARG DEBIAN_VERSION="bullseye"

ARG VERSION="0.0.1"


FROM rust:${RUST_VERSION}-${DEBIAN_VERSION} as builder

ARG VERSION

WORKDIR /code
COPY . /code

SHELL ["/bin/bash", "-c", "-o", "pipefail"]
# remove leading v from version number and use it as the crate version
RUN CRATE_VERSION=$(echo ${VERSION} | sed "s/v\(.*\)/\1/") &&\
    sed -ri "s/^version = \".*\"/version = \"${CRATE_VERSION}\"/" Cargo.toml &&\
    cargo build --release


FROM debian:${DEBIAN_VERSION}

ARG VERSION
ARG BUILD_DATE
ARG GIT_HASH

LABEL org.opencontainers.image.version="$VERSION"
LABEL org.opencontainers.image.created="$BUILD_DATE"
LABEL org.opencontainers.image.revision="$GIT_HASH"
LABEL org.opencontainers.image.title="statuspage-exporter"
LABEL org.opencontainers.image.description="Exports Statuspage.io status as Prometheus metrics."
LABEL org.opencontainers.image.vendor="Vlad Vasiliu"
LABEL org.opencontainers.image.source="https://github.com/vladvasiliu/statuspage-exporter"
LABEL org.opencontainers.image.authors="Vlad Vasiliu"

EXPOSE 9925
ENV STATUSPAGE_EXPORTER_LISTEN="0.0.0.0:9925"

RUN apt-get update && apt-get install --no-install-recommends -y curl=7.74.0-1.3+deb11u1 ca-certificates=20210119 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /code/target/release/statuspage-exporter /

CMD ["/statuspage-exporter"]
