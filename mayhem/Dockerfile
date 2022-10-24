FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /circomspect
WORKDIR /circomspect/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /circomspect/fuzz/target/x86_64-unknown-linux-gnu/release/circomspect-parse-fuzz /