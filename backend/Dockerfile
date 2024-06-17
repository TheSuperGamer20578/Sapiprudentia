FROM rust AS builder

WORKDIR /backend

COPY . .
RUN cargo build --release

FROM debian:stable-slim

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

COPY --from=builder /backend/target/release/sapiprudentia /

ENTRYPOINT ["/sapiprudentia"]
