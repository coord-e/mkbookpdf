FROM rust

COPY . /build
WORKDIR /build

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --all-features --target=x86_64-unknown-linux-musl

FROM alpine

COPY --from=0 /build/target/x86_64-unknown-linux-musl/release/mkbooklet /usr/bin/mkbooklet
CMD ["/usr/bin/mkbooklet"]