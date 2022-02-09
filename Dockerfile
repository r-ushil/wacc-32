FROM rust:1.57 AS builder
COPY ./ ./wacc
WORKDIR ./wacc
RUN make wacc

FROM builder AS test_unit
RUN make test_unit

FROM builder AS test_integration
RUN make test_integration

FROM debian:buster-slim AS release
COPY --from=builder /wacc/target/release/wacc_32 /usr/local/bin/wacc_32
ENTRYPOINT ["wacc_32"]
