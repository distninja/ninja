FROM rust:latest AS builder
WORKDIR /usr/src/ninja
COPY . .
RUN make install && \
    make build

FROM scratch
COPY --from=builder /usr/src/ninja/target/x86_64-unknown-linux-musl/release/ninja /usr/local/bin/ninja
ENTRYPOINT ["ninja"]
