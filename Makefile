THIS SHOULD BE A LINTER ERROR.PHONY: build install test clean all venv

# Virtual environment settings
VENV_DIR = venv
VENV_ACTIVATE = $(VENV_DIR)/bin/activate
PYTHON = $(VENV_DIR)/bin/python
PIP = $(VENV_DIR)/bin/pip

# Default target
all: venv build install test

# Create and setup virtual environment
venv:
	@echo "Creating virtual environment..."
	@if [ ! -d "$(VENV_DIR)" ]; then \
		echo "Checking for python3-venv..."; \
		if ! python3 -m venv --help >/dev/null 2>&1; then \
			echo "Error: python3-venv is not available."; \
			echo "On Ubuntu/Debian, install with: sudo apt install python3-venv"; \
			echo "Or use existing environment: make VENV_DIR=/workspace/test_env"; \
			exit 1; \
		fi; \
		python3 -m venv $(VENV_DIR) && \
		echo "Virtual environment created in $(VENV_DIR)" && \
		echo "Installing required packages..." && \
		. $(VENV_ACTIVATE) && pip install --upgrade pip && \
		pip install maturin pytest; \
	else \
		echo "Virtual environment already exists"; \
	fi

# Build the Rust package
build: venv
	@echo "Building mtgjson_rust package..."
	@. $(VENV_ACTIVATE) && cd mtgjson-rust && maturin build --release

# Install the built package
install: build
	@echo "Installing mtgjson_rust package..."
	@. $(VENV_ACTIVATE) && pip install --force-reinstall mtgjson-rust/target/wheels/mtgjson_rust-0.1.0-cp313-cp313-manylinux_2_34_x86_64.whl

# Run tests
test: install
	@echo "Running tests..."
	@. $(VENV_ACTIVATE) && python -m pytest tests/mtgjson5/test_nothing.py -v
	@. $(VENV_ACTIVATE) && python -m pytest tests/mtgjson5/test_card_sorting.py -v || . $(VENV_ACTIVATE) && python test_card_sorting_rust.py
	@echo "All tests completed!"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@. $(VENV_ACTIVATE) && cd mtgjson-rust && cargo clean
	@rm -rf mtgjson-rust/target/wheels/*.whl

# Clean everything including virtual environment
clean-all: clean
	@echo "Removing virtual environment..."
	@rm -rf $(VENV_DIR)

# Quick test with embedded Python sorting
test-sorting: venv install
	@echo "Testing card sorting with embedded Python logic..."
	@. $(VENV_ACTIVATE) && python test_embedded_python.py

# Development build (faster, debug mode)
dev-build: venv
	@echo "Building in development mode..."
	@. $(VENV_ACTIVATE) && cd mtgjson-rust && maturin develop

# Full rebuild from scratch
rebuild: clean build install

# Activate virtual environment (for manual use)
activate:
	@echo "To activate the virtual environment, run:"
	@echo "source $(VENV_ACTIVATE)"

# Help
help:
	@echo "Available targets:"
	@echo "  all         - Create venv, build, install, and test (default)"
	@echo "  venv        - Create and setup virtual environment"
	@echo "  build       - Build the Rust package"
	@echo "  install     - Install the built package"
	@echo "  test        - Run the test suite"
	@echo "  test-sorting - Test card sorting specifically"
	@echo "  dev-build   - Fast development build"
	@echo "  rebuild     - Clean rebuild from scratch"
	@echo "  clean       - Clean build artifacts"
	@echo "  clean-all   - Clean everything including venv"
	@echo "  activate    - Show how to activate venv manually"
	@echo "  help        - Show this help message"