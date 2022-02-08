FROM rust:1.57 AS builder
COPY ./ ./wacc
WORKDIR ./wacc
RUN make

FROM builder AS test
RUN make test

FROM builder AS release
ENTRYPOINT ["/bin/sh", "-c", "./compile"]
