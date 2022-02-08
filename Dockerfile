FROM rust:1.57 AS builder
COPY ./ ./wacc
WORKDIR ./wacc
RUN make

FROM builder AS test
RUN make test

FROM debian:buster-slim AS release
WORKDIR ./wacc
COPY --from=builder /wacc/target/release/wacc_32 ./target/release/wacc_32
COPY --from=builder /wacc/compile ./compile
ENTRYPOINT ["/bin/sh", "-c", "./compile"]
