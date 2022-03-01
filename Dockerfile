FROM rust:latest AS builder
WORKDIR /usr/src/ninja
COPY . .
RUN make install && \
    make build

FROM gcr.io/distroless/base-debian11
COPY --from=builder /usr/src/ninja/target/release/ninja /usr/local/bin/ninja
USER nonroot:nonroot
ENTRYPOINT ["ninja"]
