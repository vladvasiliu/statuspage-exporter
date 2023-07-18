ARG RUST_VERSION="1.70.0"
ARG DEBIAN_VERSION="bookworm"


FROM rust:${RUST_VERSION}-${DEBIAN_VERSION} as builder

RUN cargo install cargo-auditable
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt-get install -y musl-tools

WORKDIR /code
COPY . /code

RUN cargo --config net.git-fetch-with-cli=true auditable build --release --target x86_64-unknown-linux-musl

FROM scratch

LABEL org.opencontainers.image.authors="Vlad Vasiliu"

EXPOSE 9925
ENV STATUSPAGE_EXPORTER_LISTEN="0.0.0.0:9925"

COPY --from=builder /code/target/x86_64-unknown-linux-musl/release/statuspage-exporter /

CMD ["/statuspage-exporter"]
