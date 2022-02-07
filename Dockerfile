FROM rust:1.57 AS builder
COPY ./ ./wacc
WORKDIR ./wacc
RUN make

FROM builder AS test
WORKDIR ./wacc
RUN make test
