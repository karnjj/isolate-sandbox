FROM rust:1.90-slim-bookworm AS rust-base
RUN apt-get update && apt-get install -y curl

FROM debian:bookworm-slim AS debian-base
RUN apt-get update && apt-get install -y curl

FROM rust-base AS deps

WORKDIR /app

# Copy only the Cargo.toml and Cargo.lock files to speed up the build process
COPY Cargo.toml Cargo.lock ./

# Build the dependencies
RUN mkdir src \
    && echo "// dummy file" > src/lib.rs \
    && cargo build --release

FROM deps AS builder

WORKDIR /app

# Copy the source code
COPY src src

# Build the application
RUN cargo build --locked --release 

FROM debian-base AS runtime

# install python 
RUN apt-get update && apt-get install -y python3 python3-pip python3-venv

# install isolate
RUN curl https://www.ucw.cz/isolate/debian/signing-key.asc | gpg --dearmor -o /etc/apt/keyrings/isolate.gpg
RUN echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/isolate.gpg] http://www.ucw.cz/isolate/debian/ bookworm-isolate main" | tee /etc/apt/sources.list.d/isolate.list
RUN apt update && apt install -y isolate
RUN sed -i 's@^cg_root .*@cg_root = /sys/fs/cgroup@' /etc/isolate

FROM runtime

WORKDIR /app

RUN apt install -y sudo

# Copy config folder
COPY config config

# Copy the application binary
COPY --from=builder /app/target/release/isolate-sandbox ./isolate-sandbox

ENV RUST_LOG=info

# DEBUG RUN FOREVER
# CMD ["tail", "-f", "/dev/null"]

# Run the application
CMD ["./isolate-sandbox"]


