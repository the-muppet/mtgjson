# Global ARG declarations for use in FROM statements
ARG RUST_VERSION=1.75
ARG PYTHON_VERSION=3.11

# Multi-stage build: Rust compilation stage
FROM rust:${RUST_VERSION}-slim-buster as rust-builder

# Re-declare ARGs for this stage
ARG RUST_VERSION=1.75
ARG MATURIN_VERSION=1.4.0
ARG BUILD_MODE=release

WORKDIR /build

# Install required system dependencies for Rust compilation
# patchelf is essential for maturin wheel building
RUN apt update && apt install -y \
    python3-dev \
    python3-pip \
    pkg-config \
    libssl-dev \
    patchelf \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install maturin for building Python wheels from Rust
# Pin the version for reproducible builds
RUN pip3 install maturin==${MATURIN_VERSION}

# Copy only Cargo files first for better layer caching
COPY ./mtgjson-rust/Cargo.toml ./mtgjson-rust/Cargo.lock ./mtgjson-rust/

# Pre-download and compile dependencies (this layer will be cached)
WORKDIR /build/mtgjson-rust
RUN cargo fetch

# Copy Rust source code
COPY ./mtgjson-rust/src ./src

# Build the Python wheel with conditional build mode
RUN if [ "$BUILD_MODE" = "debug" ]; then \
      maturin build --out /build/wheels; \
    else \
      maturin build --release --out /build/wheels; \
    fi

# Verify the wheel was built
RUN ls -la /build/wheels/ && \
    python3 -c "import os; wheels = [f for f in os.listdir('/build/wheels') if f.endswith('.whl')]; print(f'Built wheels: {wheels}'); assert wheels, 'No wheels found!'"

# Testing stage (optional target)
FROM rust-builder as test
RUN cargo test --release

# Final stage: Python application
FROM python:${PYTHON_VERSION}-slim-buster as final

# Re-declare ARGs for this stage
ARG PYTHON_VERSION=3.11
ARG INSTALL_DEV_TOOLS=false
ARG BUILD_MODE=release

WORKDIR /mtgjson

# Install system dependencies
# Include patchelf in final image for runtime wheel operations
RUN apt update \
    && apt install -y --no-install-recommends \
        git \
        bzip2 \
        xz-utils \
        zip \
        htop \
        patchelf \
    && if [ "$INSTALL_DEV_TOOLS" = "true" ]; then \
        apt install -y --no-install-recommends \
            curl \
            vim \
            strace \
            gdb; \
    fi \
    && apt purge -y --auto-remove \
    && rm -rf /var/lib/apt/lists/*

# Copy application code
COPY ./mtgjson5 ./mtgjson5
COPY ./requirements.txt ./requirements.txt

# Copy the built Rust wheel from the builder stage
COPY --from=rust-builder /build/wheels/*.whl ./

# Install Python dependencies and the Rust module
RUN pip3 install --no-cache-dir -r ./requirements.txt pip && \
    pip3 install --no-cache-dir *.whl && \
    rm -f *.whl

# Verify the Rust module can be imported
RUN python3 -c "import mtgjson_rust; print('✓ mtgjson_rust module imported successfully')"

# Add healthcheck
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD python3 -c "import mtgjson_rust; print('healthy')" || exit 1

# Set default target for multi-stage builds
FROM final

ENTRYPOINT ["python3", "-m", "mtgjson5", "--use-envvars"]
