FROM rust:1.36-slim as build

RUN apt-get update \
    && apt-get install -y musl-dev musl-tools \
    && rustup target add x86_64-unknown-linux-musl

WORKDIR /opt/tfc

# Downloading and building dependencies takes a while, so do it for a dummy
# binary with the same dependencies first, so they're cached if Cargo.* don't
# change.
RUN USER=dummy cargo init
ADD Cargo.toml Cargo.lock ./
RUN cargo build --release --target x86_64-unknown-linux-musl

# Now build for real. Touch main.rs so that cargo sees it has changed.
ADD . .
RUN touch src/main.rs \
    && cargo build --release --target x86_64-unknown-linux-musl \
    && strip target/x86_64-unknown-linux-musl/release/they-fight-crime

FROM scratch

COPY --from=build /opt/tfc/target/x86_64-unknown-linux-musl/release/they-fight-crime /opt/
ADD data.json /opt/

ENTRYPOINT [ "/opt/they-fight-crime" ]
