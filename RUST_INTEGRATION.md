# Rust Module Integration

This document explains how the Rust module is integrated into the MTGJSON build system.

## Overview

The MTGJSON project includes a high-performance Rust module (`mtgjson-rust`) that provides:
- Native Python bindings for MTGJSON data structures
- High-performance parallel processing capabilities
- Optimized data serialization and deserialization
- Fast computational modules for price building and output generation

## Docker Build (Automatic)

The Docker build automatically compiles and installs the Rust module with no additional steps required:

```bash
# Build the Docker image (includes Rust compilation)
docker build -t mtgjson .

# Run the container
docker run mtgjson
```

The multi-stage Docker build:
1. **Stage 1**: Compiles the Rust module using the official Rust image
2. **Stage 2**: Copies the compiled Python wheel to the final Python image
3. **Result**: Clean, optimized final image with the Rust module installed

## Local Development

For local development, you can build the Rust module using the provided build script:

### Prerequisites
- Python 3.8+
- Rust toolchain (install from https://rustup.rs/)

### Building the Module

```bash
# Build and install in development mode (recommended for development)
python build_rust.py

# Build in debug mode (faster compilation, slower runtime)
python build_rust.py --mode debug

# Build a distributable wheel
python build_rust.py --wheel

# Check if required tools are installed
python build_rust.py --check-only
```

### Manual Building with Maturin

If you prefer to use maturin directly:

```bash
# Install maturin
pip install maturin

# Navigate to the Rust directory
cd mtgjson-rust

# Build and install in development mode
maturin develop --release

# Or build a wheel
maturin build --release
```

## Using the Rust Module

Once built and installed, the Rust module can be imported in Python:

```python
import mtgjson_rust

# Use Rust-accelerated data structures
card = mtgjson_rust.MtgjsonCard(...)
prices = mtgjson_rust.MtgjsonPrices(...)

# Use high-performance processors
parallel_processor = mtgjson_rust.ParallelProcessor()
price_builder = mtgjson_rust.PriceBuilder()
```

## Performance Benefits

The Rust module provides significant performance improvements:

- **Parallel Processing**: Async/await based parallel processing using Tokio
- **Memory Efficiency**: Zero-copy serialization where possible
- **Type Safety**: Compile-time guarantees prevent runtime errors
- **Native Speed**: Near-C performance for computational intensive operations

## Troubleshooting

### Docker Build Issues

If the Docker build fails during Rust compilation:

1. **Increase Docker memory**: Rust compilation can be memory-intensive
   ```bash
   # Increase Docker Desktop memory allocation to 4GB+
   ```

2. **Network issues**: Ensure Docker can access crates.io
   ```bash
   # Test network connectivity
   docker run --rm rust:1.75-slim-buster ping -c 1 crates.io
   ```

### Local Build Issues

1. **Rust not found**:
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Python development headers missing**:
   ```bash
   # Ubuntu/Debian
   sudo apt install python3-dev
   
   # macOS
   xcode-select --install
   ```

3. **Compilation errors**:
   ```bash
   # Clean build
   cd mtgjson-rust
   cargo clean
   cd ..
   python build_rust.py
   ```

## Architecture

The integration follows these principles:

- **Zero Configuration**: Docker build "just works" without additional setup
- **Development Friendly**: Easy local building with helpful error messages
- **Clean Separation**: Rust code is isolated in its own directory
- **Fallback Compatible**: Python-only functionality remains available

## Build Optimization

The Docker build is optimized for:
- **Layer Caching**: Dependencies are installed before copying source code
- **Multi-stage Build**: Keeps final image size small
- **Parallel Builds**: Rust compilation uses all available CPU cores
- **Reproducible Builds**: Pinned dependency versions ensure consistency

## File Structure

```
mtgjson-v5/
├── mtgjson-rust/           # Rust module source
│   ├── src/               # Rust source files
│   ├── Cargo.toml         # Rust dependencies
│   └── Cargo.lock         # Locked dependencies
├── Dockerfile             # Multi-stage build configuration
├── build_rust.py          # Local development build script
├── .dockerignore          # Optimized build context
└── RUST_INTEGRATION.md    # This documentation
```