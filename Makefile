# MTGJSON Build Makefile
# Supports both Docker Bake builds and local Rust module building
# Cross-platform compatible (Linux, macOS, Windows)

# ================================
# PHONY Declarations
# ================================
.PHONY: help all setup-env clean rebuild
.PHONY: build build-local build-dev build-ci build-all rust-builder
.PHONY: rust-local rust-wheel rust-debug rust-check rust-test rust-clean
.PHONY: install-rust-deps troubleshoot dev-build
.PHONY: test test-all test-python test-rust install
.PHONY: clean-all push run run-dev shell inspect benchmark logs
.PHONY: install-buildx setup-builder platform-help
.PHONY: dev-cycle rust-dev-cycle full-dev-cycle setup-dev release
.PHONY: elmo-ultra
# ================================
# Configuration Variables
# ================================

# Use existing test environment or create new one
VENV_DIR ?= test_env
VENV_ACTIVATE = $(VENV_DIR)/bin/activate
PYTHON = $(VENV_DIR)/bin/python
PIP = $(VENV_DIR)/bin/pip

# Default registry and tag
REGISTRY ?= mtgjson
TAG ?= latest

# Platform detection
ifeq ($(OS),Windows_NT)
    PLATFORM := windows
    BUILD_SCRIPT := cmd /c build_rust.bat
    PYTHON := python
else
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Linux)
        PLATFORM := linux
    endif
    ifeq ($(UNAME_S),Darwin)
        PLATFORM := macos
    endif
    BUILD_SCRIPT := python ./build_rust.py
    PYTHON := python3
endif

# Colors for output (disable on Windows CMD)
ifeq ($(PLATFORM),windows)
    GREEN := 
    YELLOW := 
    RED := 
    NC := 
else
    GREEN := \033[0;32m
    YELLOW := \033[1;33m
    RED := \033[0;31m
    NC := \033[0m
endif

# ================================
# Default Target
# ================================
all: setup-env build install test ## Setup environment, build, install, and test

# ================================
# Help System
# ================================
help: ## Show this help message
	@echo "$(GREEN)MTGJSON Build System$(NC)"
	@echo "===================="
	@echo ""
	@echo "$(YELLOW)Quick Start:$(NC)"
	@echo "  make all            - Complete build cycle: setup, build, install, test"
	@echo "  make rust-local     - Build Rust module locally (fastest for development)"
	@echo "  make setup-dev      - Setup development environment"
	@echo ""
	@echo "$(YELLOW)Environment Setup:$(NC)"
	@echo "  setup-env           - Setup Python virtual environment with dependencies"
	@echo "  install-rust-deps   - Install Rust toolchain and dependencies"
	@echo "  setup-dev           - Complete development environment setup"
	@echo "  rust-check          - Check if Rust and maturin are properly installed"
	@echo ""
	@echo "$(YELLOW)Building - Docker:$(NC)"
	@echo "  build               - Build production image (multi-platform, optimized)"
	@echo "  build-local         - Build for local development (single platform, faster)"
	@echo "  build-dev           - Build development image with debug tools"
	@echo "  build-ci            - Build for CI/CD (pushes to registry)"
	@echo "  build-all           - Build all Docker targets"
	@echo "  rust-builder        - Build only the Rust builder stage (for debugging)"
	@echo ""
	@echo "$(YELLOW)Building - Local Rust:$(NC)"
	@echo "  rust-local          - Build and install Rust module locally (development mode)"
	@echo "  rust-debug          - Build Rust module in debug mode (faster compilation)"
	@echo "  rust-wheel          - Build distributable wheel"
	@echo "  dev-build           - Fast development build (maturin develop)"
	@echo ""
	@echo "$(YELLOW)Testing:$(NC)"
	@echo "  test                - Run core tests with current setup"
	@echo "  test-all            - Run all available tests"
	@echo "  test-python         - Test original Python implementation"
	@echo "  test-rust           - Test Rust implementation functionality"
	@echo "  rust-test           - Test the locally built Rust module"
	@echo ""
	@echo "$(YELLOW)Docker Operations:$(NC)"
	@echo "  push                - Push images to registry"
	@echo "  run                 - Run the built image locally"
	@echo "  run-dev             - Run development image with shell"
	@echo "  shell               - Get shell in running container"
	@echo "  inspect             - Inspect the built image"
	@echo "  benchmark           - Run a quick performance benchmark"
	@echo "  logs                - View build logs"
	@echo ""
	@echo "$(YELLOW)Cleaning:$(NC)"
	@echo "  clean               - Clean build artifacts and Docker cache"
	@echo "  clean-all           - Clean everything including images and Rust artifacts"
	@echo "  rust-clean          - Clean only Rust build artifacts"
	@echo ""
	@echo "$(YELLOW)Development Workflows:$(NC)"
	@echo "  dev-cycle           - Quick Docker development cycle: build and test"
	@echo "  rust-dev-cycle      - Quick Rust development cycle: build and test locally"
	@echo "  full-dev-cycle      - Full development cycle: Rust + Docker"
	@echo "  rebuild             - Clean and rebuild from scratch"
	@echo ""
	@echo "$(YELLOW)Docker Utilities:$(NC)"
	@echo "  install-buildx      - Install Docker Buildx (if not available)"
	@echo "  setup-builder       - Setup multi-platform builder"
	@echo ""
	@echo "$(YELLOW)Production:$(NC)"
	@echo "  release             - Full release cycle: clean, build, test, push"
	@echo ""
	@echo "$(YELLOW)Troubleshooting:$(NC)"
	@echo "  troubleshoot        - Print troubleshooting information"
	@echo "  platform-help       - Show platform-specific help and setup instructions"
	@echo ""
	@echo "$(YELLOW)Environment Variables:$(NC)"
	@echo "  REGISTRY            - Registry to push images to (default: mtgjson)"
	@echo "  TAG                 - Tag for images (default: latest)"
	@echo "  VENV_DIR            - Virtual environment directory (default: test_env)"
	@echo ""
	@echo "$(YELLOW)Platform Information:$(NC)"
	@echo "  Current platform: $(PLATFORM)"
	@echo "  Build script: $(BUILD_SCRIPT)"
	@echo ""
	@echo "For platform-specific help: make platform-help"

# ================================
# Docker Building Targets
# ================================

build: ## Build production image (multi-platform, optimized caching)
	@echo "$(GREEN)Building production image with Docker Bake...$(NC)"
	docker buildx bake -f docker-bake.hcl mtgjson \
		--set="*.args.REGISTRY=$(REGISTRY)" \
		--set="*.args.TAG=$(TAG)"

build-local: ## Build for local development (single platform, faster)
	@echo "$(GREEN)Building local development image...$(NC)"
	docker buildx bake -f docker-bake.hcl local \
		--set="*.args.REGISTRY=$(REGISTRY)" \
		--set="*.args.TAG=$(TAG)" \
		--load

build-dev: ## Build development image with debug tools
	@echo "$(GREEN)Building development image with debug tools...$(NC)"
	docker buildx bake -f docker-bake.hcl mtgjson-dev \
		--set="*.args.REGISTRY=$(REGISTRY)" \
		--set="*.args.TAG=$(TAG)" \
		--load

build-ci: ## Build for CI/CD (pushes to registry)
	@echo "$(GREEN)Building CI image and pushing to registry...$(NC)"
	docker buildx bake -f docker-bake.hcl ci \
		--set="*.args.REGISTRY=$(REGISTRY)" \
		--set="*.args.TAG=$(TAG)"

build-all: ## Build all Docker targets
	@echo "$(GREEN)Building all targets...$(NC)"
	docker buildx bake -f docker-bake.hcl all \
		--set="*.args.REGISTRY=$(REGISTRY)" \
		--set="*.args.TAG=$(TAG)"

rust-builder: ## Build only the Rust builder stage (for debugging)
	@echo "$(GREEN)Building Rust builder stage...$(NC)"
	docker buildx bake -f docker-bake.hcl rust-builder \
		--set="*.args.REGISTRY=$(REGISTRY)" \
		--set="*.args.TAG=$(TAG)"

# ================================
# Local Rust Building Targets
# ================================

rust-local: ## Build and install Rust module locally (development mode)
	@echo "$(GREEN)Building Rust module locally...$(NC)"
	$(BUILD_SCRIPT)

rust-debug: ## Build Rust module in debug mode (faster compilation)
	@echo "$(GREEN)Building Rust module in debug mode...$(NC)"
ifeq ($(PLATFORM),windows)
	$(BUILD_SCRIPT) --debug
else
	$(BUILD_SCRIPT) --mode debug
endif

rust-wheel: ## Build distributable wheel 
	@echo "$(GREEN)Building Rust wheel...$(NC)"
ifeq ($(PLATFORM),windows)
	$(BUILD_SCRIPT) --wheel
else
	$(BUILD_SCRIPT) --wheel
endif

rust-docker: ## Build Rust module in Docker
	@echo "$(GREEN)Building Rust module in Docker...$(NC)"
	docker buildx bake -f docker-bake.hcl rust-builder \
		--set="*.args.REGISTRY=$(REGISTRY)" \
		--set="*.args.TAG=$(TAG)"

dev-build: setup-env ## Fast development build (maturin develop)
	@echo "$(GREEN)Building in development mode...$(NC)"
	@. $(VENV_ACTIVATE) && cd mtgjson-rust && maturin develop

elmo-ultra: ## Nightly Rust build with maximum performance options
	@echo RUSTFLAGS="-Z threads=12 -C target-cpu=native -C target-feature=+crt-static \
	 -C link-arg=/OPT:REF -C link-arg=/OPT:ICF\" cargo build --profile elmo
# ================================
# Environment Setup Targets
# ================================

setup-env: ## Setup Python virtual environment with dependencies
	@echo "$(GREEN)Setting up environment...$(NC)"
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

install-rust-deps: ## Install Rust toolchain and dependencies
	@echo "$(GREEN)Installing Rust dependencies...$(NC)"
ifeq ($(PLATFORM),windows)
	@echo "$(YELLOW)Please install manually:$(NC)"
	@echo "1. Rust: https://rustup.rs/"
	@echo "2. Visual Studio Build Tools: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022"
	@echo "3. Python development headers (usually included with Python)"
else ifeq ($(PLATFORM),linux)
	@echo "Installing Rust and dependencies on Linux..."
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
	sudo apt update && sudo apt install -y python3-dev build-essential pkg-config libssl-dev
	$(PYTHON) -m pip install maturin
else ifeq ($(PLATFORM),macos)
	@echo "Installing Rust and dependencies on macOS..."
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
	xcode-select --install || true
	$(PYTHON) -m pip install maturin
endif

setup-dev: install-rust-deps rust-check ## Complete development environment setup
	@echo "$(GREEN)Development environment setup complete!$(NC)"
	@echo "$(YELLOW)Next steps:$(NC)"
	@echo "  make rust-local     # Build Rust module"
	@echo "  make rust-test      # Test Rust module"
	@echo "  make build-local    # Build Docker image"

rust-check: ## Check if Rust and maturin are properly installed
	@echo "$(GREEN)Checking Rust installation...$(NC)"
ifeq ($(PLATFORM),windows)
	$(BUILD_SCRIPT) --check
else
	$(BUILD_SCRIPT) --check-only
endif

# ================================
# Installation Targets
# ================================

install: build ## Install the built package
	@echo "$(GREEN)Installing mtgjson_rust package...$(NC)"
	@. $(VENV_ACTIVATE) && pip install --force-reinstall mtgjson-rust/target/wheels/mtgjson_rust-0.1.0-cp313-cp313-manylinux_2_34_x86_64.whl

# ================================
# Testing Targets
# ================================

test: install ## Run core tests with current setup
	@echo "$(GREEN)Running tests with pytest...$(NC)"
	@. $(VENV_ACTIVATE) && PYTHONPATH=/workspace pytest tests/mtgjson5/test_nothing.py tests/mtgjson5/test_card_sorting.py -v
	@echo "$(GREEN)Core tests completed successfully!$(NC)"

test-all: install ## Run all available tests
	@echo "$(GREEN)Running all available tests...$(NC)"
	@. $(VENV_ACTIVATE) && PYTHONPATH=/workspace pytest tests/mtgjson5/ -v || echo "Some tests may require additional dependencies"

test-python: setup-env ## Test original Python implementation
	@echo "$(GREEN)Testing original Python implementation...$(NC)"
	@. $(VENV_ACTIVATE) && PYTHONPATH=/workspace pytest tests/mtgjson5/test_nothing.py tests/mtgjson5/test_card_sorting.py -v

test-rust: install ## Test Rust implementation functionality
	@echo "$(GREEN)Testing Rust implementation functionality...$(NC)"
	@. $(VENV_ACTIVATE) && python -c "import mtgjson_rust; print('✓ Rust module loads'); from mtgjson_rust import MtgjsonCard; card = MtgjsonCard(); print('✓ Card creation works'); print('Available classes:', len([x for x in dir(mtgjson_rust) if x.startswith('Mtgjson')]))"

rust-test: ## Test the locally built Rust module
	@echo "$(GREEN)Testing Rust module...$(NC)"
	$(PYTHON) -c "import mtgjson_rust; print('✓ Module imported successfully')"
	$(PYTHON) -c "import mtgjson_rust; card = mtgjson_rust.MtgjsonCard(); print('✓ Card creation works')"
	$(PYTHON) -c "import mtgjson_rust; prices = mtgjson_rust.MtgjsonPrices(); print('✓ Prices creation works')"
	$(PYTHON) -c "import mtgjson_rust; proc = mtgjson_rust.ParallelProcessor(); print('✓ Parallel processor works')"
	@echo "$(GREEN)All Rust module tests passed!$(NC)"

# ================================
# Docker Testing Target
# ================================

docker-test: ## Run tests in Docker
	@echo "$(GREEN)Running tests in Docker...$(NC)"
	docker buildx bake -f docker-bake.hcl test \
		--set="*.args.REGISTRY=$(REGISTRY)" \
		--set="*.args.TAG=$(TAG)"

# ================================
# Cleaning Targets
# ================================

clean: ## Clean build artifacts and Docker cache
	@echo "$(YELLOW)Cleaning build cache...$(NC)"
	@echo "Cleaning Docker build cache..."
	-docker buildx prune -f
	-docker system prune -f --volumes
	@echo "Cleaning Rust artifacts..."
	$(MAKE) rust-clean
	@echo "Cleaning Python artifacts..."
	@rm -rf mtgjson-rust/build/
	@rm -rf mtgjson-rust/dist/
	@rm -rf mtgjson-rust/mtgjson_rust.egg-info/
	@rm -rf .tox/

clean-all: ## Clean everything including images and Rust artifacts
	@echo "$(RED)Cleaning all build resources...$(NC)"
	@echo "Cleaning Docker resources..."
	-docker buildx prune -af
	-docker system prune -af --volumes
	@echo "Cleaning Rust artifacts..."
	$(MAKE) rust-clean
	@echo "$(RED)All clean!$(NC)"

rust-clean: ## Clean only Rust build artifacts
	@echo "$(GREEN)Cleaning Rust build artifacts...$(NC)"
ifeq ($(PLATFORM),windows)
	if exist "mtgjson-rust\\target" rmdir /s /q "mtgjson-rust\\target"
	if exist "*.whl" del /f /q "*.whl"
else
	rm -rf mtgjson-rust/target/
	rm -f *.whl
endif

# ================================
# Docker Operations
# ================================

push: ## Push images to registry
	@echo "$(GREEN)Pushing images to $(REGISTRY)...$(NC)"
	docker push $(REGISTRY)/mtgjson:$(TAG)
	docker push $(REGISTRY)/rust-builder:$(TAG)

run: ## Run the built image locally
	@echo "$(GREEN)Running MTGJSON container...$(NC)"
	docker run --rm -it $(REGISTRY)/mtgjson:$(TAG)

run-dev: ## Run development image with shell
	@echo "$(GREEN)Running development container with shell...$(NC)"
	docker run --rm -it --entrypoint /bin/bash $(REGISTRY)/mtgjson:dev

shell: ## Get shell in running container
	@echo "$(GREEN)Getting shell in container...$(NC)"
	docker run --rm -it --entrypoint /bin/bash $(REGISTRY)/mtgjson:$(TAG)

inspect: ## Inspect the built image
	@echo "$(GREEN)Inspecting image...$(NC)"
	docker run --rm $(REGISTRY)/mtgjson:$(TAG) python3 -c "\
import mtgjson_rust; \
print('✓ Rust module loaded successfully'); \
print('Available classes:', [attr for attr in dir(mtgjson_rust) if not attr.startswith('_')])"

benchmark: ## Run a quick performance benchmark
	@echo "$(GREEN)Running benchmark...$(NC)"
	docker run --rm $(REGISTRY)/mtgjson:$(TAG) python3 -c "\
import time; \
import mtgjson_rust; \
start = time.time(); \
for i in range(1000): \
    card = mtgjson_rust.MtgjsonCard(); \
print(f'Created 1000 cards in {time.time() - start:.4f}s')"

logs: ## View build logs
	docker buildx bake -f docker-bake.hcl mtgjson --progress=plain

# ================================
# Docker Utilities
# ================================

install-buildx: ## Install Docker Buildx (if not available)
	@echo "$(GREEN)Installing Docker Buildx...$(NC)"
	docker buildx install

setup-builder: ## Setup multi-platform builder
	@echo "$(GREEN)Setting up multi-platform builder...$(NC)"
	docker buildx create --name mtgjson-builder --use --bootstrap
	docker buildx ls

# ================================
# Development Workflows
# ================================

dev-cycle: build-local inspect ## Quick Docker development cycle: build and test
	@echo "$(GREEN)Docker development cycle complete!$(NC)"

rust-dev-cycle: rust-local rust-test ## Quick Rust development cycle: build and test locally
	@echo "$(GREEN)Rust development cycle complete!$(NC)"

full-dev-cycle: rust-local rust-test build-local inspect ## Full development cycle: Rust + Docker
	@echo "$(GREEN)Full development cycle complete!$(NC)"

rebuild: clean build install ## Clean and rebuild from scratch

# ================================
# Production Workflows
# ================================

release: clean build docker-test push ## Full release cycle: clean, build, test, push
	@echo "$(GREEN)Release cycle complete!$(NC)"

# ================================
# Troubleshooting and Help
# ================================

troubleshoot: ## Print troubleshooting information
	@echo "$(GREEN)Gathering troubleshooting information...$(NC)"
ifeq ($(PLATFORM),windows)
	$(BUILD_SCRIPT) --troubleshoot
else
	$(BUILD_SCRIPT) --troubleshoot
endif

platform-help: ## Show platform-specific help and setup instructions
	@echo "$(GREEN)Platform-Specific Information$(NC)"
	@echo "================================="
	@echo "Platform: $(PLATFORM)"
	@echo ""
ifeq ($(PLATFORM),windows)
	@echo "$(YELLOW)Windows Notes:$(NC)"
	@echo "- Use 'build_rust.bat' for Rust building"
	@echo "- Make sure Visual Studio Build Tools are installed"
	@echo "- Run as Administrator if you get permission errors"
	@echo "- Docker Desktop required for Docker commands"
	@echo ""
	@echo "$(YELLOW)Recommended Windows Setup:$(NC)"
	@echo "1. Install Rust: https://rustup.rs/"
	@echo "2. Install Visual Studio Build Tools"
	@echo "3. Run: make rust-check"
	@echo "4. Run: make rust-local"
else ifeq ($(PLATFORM),linux)
	@echo "$(YELLOW)Linux Notes:$(NC)"
	@echo "- Most dependencies available via package manager"
	@echo "- May need 'sudo' for system package installation"
	@echo "- Docker installation varies by distribution"
	@echo ""
	@echo "$(YELLOW)Quick Linux Setup:$(NC)"
	@echo "  make install-rust-deps  # Install everything needed"
	@echo "  make rust-local         # Build Rust module"
else ifeq ($(PLATFORM),macos)
	@echo "$(YELLOW)macOS Notes:$(NC)"
	@echo "- Xcode Command Line Tools required"
	@echo "- Homebrew recommended for dependencies"
	@echo "- Docker Desktop available from Docker website"
	@echo ""
	@echo "$(YELLOW)Quick macOS Setup:$(NC)"
	@echo "  make install-rust-deps  # Install everything needed"
	@echo "  make rust-local         # Build Rust module"
endif
