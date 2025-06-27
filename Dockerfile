# Multi-stage build: Rust compilation stage
FROM rust:1.75-slim-buster as rust-builder

WORKDIR /build

# Install required system dependencies for Rust compilation
RUN apt update && apt install -y \
    python3-dev \
    python3-pip \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install maturin for building Python wheels from Rust
RUN pip3 install maturin==1.4.0

# Copy Rust source code
COPY ./mtgjson-rust ./mtgjson-rust

# Build the Python wheel
WORKDIR /build/mtgjson-rust
RUN maturin build --release --out /build/wheels

# Final stage: Python application
FROM python:3.11-slim-buster

WORKDIR /mtgjson

# Install system dependencies
RUN apt update \
    && apt install -y --no-install-recommends git bzip2 xz-utils zip htop  \
    && apt purge -y --auto-remove \
    && rm -rf /var/lib/apt/lists/*

# Copy application code
COPY ./mtgjson5 ./mtgjson5
COPY ./requirements.txt ./requirements.txt

# Copy the built Rust wheel from the builder stage
COPY --from=rust-builder /build/wheels/*.whl ./

# Install Python dependencies and the Rust module
RUN pip3 install -r ./requirements.txt pip && \
    pip3 install *.whl && \
    rm -f *.whl

ENTRYPOINT ["python3", "-m", "mtgjson5", "--use-envvars"]
