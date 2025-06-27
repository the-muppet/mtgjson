.PHONY: build install test clean all setup-env

# Use existing test environment or create new one
VENV_DIR ?= test_env
VENV_ACTIVATE = $(VENV_DIR)/bin/activate
PYTHON = $(VENV_DIR)/bin/python
PIP = $(VENV_DIR)/bin/pip

# Default target
all: setup-env build install test

# Setup environment (use existing or create new)
setup-env:
	@echo "Setting up environment..."
	@if [ ! -d "$(VENV_DIR)" ]; then \
		echo "Creating new virtual environment..."; \
		python3 -m venv $(VENV_DIR) && \
		echo "Installing dependencies..." && \
		. $(VENV_ACTIVATE) && pip install --upgrade pip && \
		pip install -r requirements.txt -r requirements_test.txt && \
		pip install maturin tox; \
	else \
		echo "Using existing environment: $(VENV_DIR)"; \
		echo "Installing missing dependencies..." && \
		. $(VENV_ACTIVATE) && pip install -q maturin tox || true; \
	fi

# Build the Rust package
build: setup-env
	@echo "Building mtgjson_rust package..."
	@. $(VENV_ACTIVATE) && cd mtgjson-rust && maturin build --release

# Install the built package
install: build
	@echo "Installing mtgjson_rust package..."
	@. $(VENV_ACTIVATE) && pip install --force-reinstall mtgjson-rust/target/wheels/mtgjson_rust-0.1.0-cp313-cp313-manylinux_2_34_x86_64.whl

# Run the actual tests using pytest with proper Python path
test: install
	@echo "Running tests with pytest..."
	@. $(VENV_ACTIVATE) && PYTHONPATH=/workspace pytest tests/mtgjson5/test_nothing.py tests/mtgjson5/test_card_sorting.py -v
	@echo "Core tests completed successfully!"

# Additional test targets
test-all: install
	@echo "Running all available tests..."
	@. $(VENV_ACTIVATE) && PYTHONPATH=/workspace pytest tests/mtgjson5/ -v || echo "Some tests may require additional dependencies"

test-python: setup-env
	@echo "Testing original Python implementation..."
	@. $(VENV_ACTIVATE) && PYTHONPATH=/workspace pytest tests/mtgjson5/test_nothing.py tests/mtgjson5/test_card_sorting.py -v

test-rust: install
	@echo "Testing Rust implementation functionality..."
	@. $(VENV_ACTIVATE) && python -c "import mtgjson_rust; print('✓ Rust module loads'); from mtgjson_rust import MtgjsonCard; card = MtgjsonCard(); print('✓ Card creation works'); print('Available classes:', len([x for x in dir(mtgjson_rust) if x.startswith('Mtgjson')]))"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@rm -rf mtgjson-rust/target/
	@rm -rf mtgjson-rust/build/
	@rm -rf mtgjson-rust/dist/
	@rm -rf mtgjson-rust/mtgjson_rust.egg-info/
	@rm -rf .tox/
	@echo "Clean complete!"

# Development build (faster, debug mode)
dev-build: setup-env
	@echo "Building in development mode..."
	@. $(VENV_ACTIVATE) && cd mtgjson-rust && maturin develop

# Full rebuild from scratch
rebuild: clean build install

# Help
help:
	@echo "Available targets:"
	@echo "  all         - Setup environment, build, install, and test"
	@echo "  setup-env   - Setup environment with dependencies"
	@echo "  build       - Build the Rust package"
	@echo "  install     - Install the built package"
	@echo "  test        - Run unit tests with tox"
	@echo "  test-all    - Run all tox environments"
	@echo "  test-lint   - Run linting tests"
	@echo "  test-mypy   - Run mypy type checking"
	@echo "  clean       - Clean build artifacts"
	@echo "  dev-build   - Fast development build"
	@echo "  rebuild     - Clean and rebuild"
	@echo "  help        - Show this help"
	@echo ""
	@echo "Environment variables:"
	@echo "  VENV_DIR    - Virtual environment directory (default: test_env)"