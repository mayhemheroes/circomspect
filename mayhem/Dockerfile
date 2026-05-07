FROM rustlang/rust:nightly AS builder

RUN cargo install cargo-fuzz

ADD . /circomspect
WORKDIR /circomspect/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /circomspect/fuzz/target/x86_64-unknown-linux-gnu/release/circomspect-parse-fuzz /
