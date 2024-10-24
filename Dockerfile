FROM rust:1.81.0 AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release


# FROM debian:bullseye-slim

# COPY --from=builder /usr/src/app/target/release/rust_api /bin/
# EXPOSE 8080
# CMD ["/bin/rust_api"]

FROM debian:bookworm-slim AS final
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
RUN apt-get -y update; apt-get -y install curl
USER appuser
COPY --from=builder /usr/src/app/target/release/rust_api /bin/
EXPOSE 8080
CMD ["/bin/rust_api"]
