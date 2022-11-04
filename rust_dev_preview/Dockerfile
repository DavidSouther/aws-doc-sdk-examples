# syntax=docker/dockerfile:1
ARG MSRV=1.58
FROM rust:$MSRV
COPY . /src/
CMD cargo test --all-targets --ignored --path /src/
