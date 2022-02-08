FROM rust:1.57 AS builder
COPY ./ ./wacc
WORKDIR ./wacc
RUN make

FROM builder AS test_unit
RUN make test_unit

FROM builder AS test_integration
RUN make test_integration

FROM debian:buster-slim AS release
COPY --from=builder /wacc/target/release/wacc_32 /wacc/target/release/wacc_32
COPY --from=builder /wacc/compile /wacc/compile
ENTRYPOINT ["/bin/sh", "-c", "/wacc/compile"]
