FROM rust

COPY . /build
WORKDIR /build

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --all-features --target=x86_64-unknown-linux-musl

FROM alpine

RUN apk add --update --no-cache cups
COPY --from=0 /build/target/x86_64-unknown-linux-musl/release/mkbookpdf /usr/bin/mkbookpdf

WORKDIR /data
ENTRYPOINT ["/usr/bin/mkbookpdf"]