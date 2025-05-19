####################################################################################################
## Builder Stage
####################################################################################################
FROM rust:latest AS builder
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

WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends jq && \
    rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --release


RUN BINARY_SERVICE_NAME=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[].targets[] | select( .kind | map(. == "bin") | any ) | .name' | head -n 1) && \
    if [ -z "${BINARY_SERVICE_NAME}" ]; then \
    echo "Error: Could not determine binary name from cargo metadata." >&2; \
    exit 1; \
    fi && \
    echo "Identified binary for service: ${BINARY_SERVICE_NAME}" && \
    SOURCE_BINARY_PATH="/app/target/release/${BINARY_SERVICE_NAME}" && \
    DEST_BINARY_PATH="/app/service" && \
    echo "Copying binary from ${SOURCE_BINARY_PATH} to ${DEST_BINARY_PATH}" && \
    cp "${SOURCE_BINARY_PATH}" "${DEST_BINARY_PATH}" && \
    echo "Stripping binary at ${DEST_BINARY_PATH}" && \
    strip -s "${DEST_BINARY_PATH}" || \
    (echo "Error: Failed during binary processing (copy or strip)." >&2 && exit 1)

####################################################################################################
## Final image Stage
####################################################################################################
FROM gcr.io/distroless/cc
USER app:app
WORKDIR /app

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

COPY --from=builder /app/service ./service

CMD ["/app/service"]
