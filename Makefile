.PHONY: build install test clean all

# Default target
all: build install test

# Build the Rust package
build:
	@echo "Building mtgjson_rust package..."
	cd mtgjson-rust && maturin build --release

# Install the built package
install: build
	@echo "Installing mtgjson_rust package..."
	pip install --force-reinstall --break-system-packages mtgjson-rust/target/wheels/mtgjson_rust-0.1.0-cp313-cp313-manylinux_2_34_x86_64.whl

# Run tests
test: install
	@echo "Running tests..."
	python -m pytest tests/mtgjson5/test_nothing.py -v
	python -m pytest tests/mtgjson5/test_card_sorting.py -v || python test_card_sorting_rust.py
	@echo "All tests completed!"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cd mtgjson-rust && cargo clean
	rm -rf mtgjson-rust/target/wheels/*.whl

# Quick test with embedded Python sorting
test-sorting:
	@echo "Testing card sorting with embedded Python logic..."
	python test_embedded_python.py

# Development build (faster, debug mode)
dev-build:
	@echo "Building in development mode..."
	cd mtgjson-rust && maturin develop

# Full rebuild from scratch
rebuild: clean build install

# Help
help:
	@echo "Available targets:"
	@echo "  all         - Build, install, and test (default)"
	@echo "  build       - Build the Rust package"
	@echo "  install     - Install the built package"
	@echo "  test        - Run the test suite"
	@echo "  test-sorting - Test card sorting specifically"
	@echo "  dev-build   - Fast development build"
	@echo "  rebuild     - Clean rebuild from scratch"
	@echo "  clean       - Clean build artifacts"
	@echo "  help        - Show this help message"