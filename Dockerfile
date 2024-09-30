#
# This is the Dockerfile for the Ruline Application.
#
# The stages are as follows:
# - ui-builder: Builds the UI.
# - 2-4 builder: Builds the application.
# - runtime: The final image that will be used to run the application.
#

#
# --------------------------------------
#
FROM node:22-alpine3.20 AS ui-builder
WORKDIR /ui

# Build the UI with pnpm.
RUN corepack enable
COPY ui/package.json ui/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile
COPY ui .
RUN pnpm build

#
# --------------------------------------
#
FROM lukemathwalker/cargo-chef:latest-rust-1-alpine3.20 AS chef
WORKDIR /app

FROM chef AS planner

# Prepare the dependencies.
COPY Cargo.toml Cargo.lock ./
COPY crates crates
RUN cargo chef prepare --bin ruline-console --recipe-path recipe.json

FROM chef AS builder

# Build the application.
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --bin ruline-console --recipe-path recipe.json
COPY Cargo.toml Cargo.lock ./
COPY crates crates
RUN cargo build --release --bin ruline-console

#
# --------------------------------------
#
FROM alpine:3.20 AS runtime
WORKDIR /app

# Install certificates.
RUN apk add --no-cache openssl ca-certificates

# Copy the built application and the UI.
COPY --from=builder /app/target/release/ruline-console /usr/local/bin/
COPY --from=ui-builder /ui/dist /app/ui/dist

# move the emails templates to the right place
RUN mv /app/ui/dist/emails /app/emails

# Create a user to run the application.
RUN addgroup -S ruline && adduser -S ruline -G ruline
USER ruline

# Expose the port.
EXPOSE 8000
ENTRYPOINT ["/usr/local/bin/ruline-console"]
