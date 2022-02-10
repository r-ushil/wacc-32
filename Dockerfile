FROM rust:1.57 AS base
WORKDIR /
RUN cargo new wacc
WORKDIR /wacc

FROM base AS dependencies
COPY Cargo.lock .
COPY Cargo.toml .
RUN cargo fetch
RUN rm -rf src/*.rs

FROM dependencies AS builder
COPY Makefile .
COPY ./src ./src
RUN make wacc

FROM builder AS test_unit
CMD make test_unit

FROM builder AS test_integration
COPY ./test_integration ./test_integration
COPY ./test_integration.sh .
CMD make test_integration

FROM debian:buster-slim AS release
COPY --from=builder /wacc/target/release/wacc_32 /usr/local/bin/wacc_32
ENTRYPOINT ["wacc_32"]
