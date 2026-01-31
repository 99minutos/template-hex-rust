####################################################################################################
## Builder Stage
####################################################################################################
FROM rust:bookworm AS builder
RUN update-ca-certificates

ENV USER=app
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

RUN apt-get update && \
    apt-get install -y protobuf-compiler jq && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --release


####################################################################################################
## Final image Stage
####################################################################################################
FROM gcr.io/distroless/cc-debian12
USER app:app
WORKDIR /app

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

COPY --from=builder /app/target/release/service ./service

CMD ["/app/service"]

