FROM rust:1.47 as builder
WORKDIR /usr/src/sflyn
COPY . .
RUN cargo build --release

FROM ubuntu:20.04
COPY --from=builder usr/src/sflyn/target/release/sflyn /usr/bin
CMD [ "sflyn", "--help" ]
