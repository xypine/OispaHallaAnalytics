FROM rustlang/rust:nightly as builder

# Make a fake Rust app to keep a cached layer of compiled crates
RUN USER=root cargo new app
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock build.rs ./
# Needs at least a main.rs file with a main function
RUN mkdir src && echo "fn main(){}" > src/main.rs
# Will build all dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo build --release

# Copy the rest
COPY . .
#RUN chmod a+rwx db/template.db
# Build (install) the actual binaries
RUN cargo install --path .



# Runtime image
FROM debian:bullseye-slim

# Install components needed for ssl
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates

RUN update-ca-certificates

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app
RUN mkdir /app/data
# Make the data directory writable
RUN chown -R app /app/data
RUN chmod a+rwx -R /app/data

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/local/cargo/bin/oispa_halla_analytics /app/oispa_halla_analytics

# Reset db url for actually running the server
#ENV DATABASE_URL="sqlite:/app/data/analytics.db"
#COPY --from=builder /usr/src/app/db/template.db /app/data/analytics.db
# RUN cp -n -r /app/data /data

# Run without tls
CMD ["/app/oispa_halla_analytics"]
