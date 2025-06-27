# MTGJSON Docker Build Makefile
# Uses Docker Bake for optimized builds with advanced caching

.PHONY: help build build-local build-dev build-ci build-all clean test push

# Default registry and tag
REGISTRY ?= mtgjson
TAG ?= latest

# Colors for output
GREEN := \033[0;32m
YELLOW := \033[1;33m
RED := \033[0;31m
NC := \033[0m # No Color

help: ## Show this help message
	@echo "$(GREEN)MTGJSON Docker Build Commands$(NC)"
	@echo "================================="
	@echo ""
	@echo "$(YELLOW)Quick Start:$(NC)"
	@echo "  make build-local    # Fast local build (single platform)"
	@echo "  make build-dev      # Development build with debug tools"
	@echo "  make build          # Production build (multi-platform)"
	@echo ""
	@echo "$(YELLOW)Available Commands:$(NC)"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  $(GREEN)%-15s$(NC) %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@echo ""
	@echo "$(YELLOW)Environment Variables:$(NC)"
	@echo "  REGISTRY=$(REGISTRY)    # Docker registry"
	@echo "  TAG=$(TAG)         # Image tag"
	@echo ""
	@echo "$(YELLOW)Examples:$(NC)"
	@echo "  REGISTRY=myregistry TAG=v1.0 make build"
	@echo "  make build-local && docker run $(REGISTRY)/mtgjson:local"

build: ## Build production image (multi-platform, optimized caching)
	@echo "$(GREEN)Building production image with Docker Bake...$(NC)"
	docker buildx bake mtgjson \
		--set="*.args.REGISTRY=$(REGISTRY)" \
		--set="*.args.TAG=$(TAG)"

build-local: ## Build for local development (single platform, faster)
	@echo "$(GREEN)Building local development image...$(NC)"
	docker buildx bake local \
		--set="*.args.REGISTRY=$(REGISTRY)" \
		--set="*.args.TAG=$(TAG)" \
		--load

build-dev: ## Build development image with debug tools
	@echo "$(GREEN)Building development image with debug tools...$(NC)"
	docker buildx bake mtgjson-dev \
		--set="*.args.REGISTRY=$(REGISTRY)" \
		--set="*.args.TAG=$(TAG)" \
		--load

build-ci: ## Build for CI/CD (pushes to registry)
	@echo "$(GREEN)Building CI image and pushing to registry...$(NC)"
	docker buildx bake ci \
		--set="*.args.REGISTRY=$(REGISTRY)" \
		--set="*.args.TAG=$(TAG)"

build-all: ## Build all targets
	@echo "$(GREEN)Building all targets...$(NC)"
	docker buildx bake all \
		--set="*.args.REGISTRY=$(REGISTRY)" \
		--set="*.args.TAG=$(TAG)"

rust-builder: ## Build only the Rust builder stage (for debugging)
	@echo "$(GREEN)Building Rust builder stage...$(NC)"
	docker buildx bake rust-builder \
		--set="*.args.REGISTRY=$(REGISTRY)" \
		--set="*.args.TAG=$(TAG)"

test: ## Run tests in Docker
	@echo "$(GREEN)Running tests...$(NC)"
	docker buildx bake test \
		--set="*.args.REGISTRY=$(REGISTRY)" \
		--set="*.args.TAG=$(TAG)"

clean: ## Clean Docker build cache
	@echo "$(YELLOW)Cleaning Docker build cache...$(NC)"
	docker buildx prune -f
	docker system prune -f --volumes

clean-all: ## Clean everything including images
	@echo "$(RED)Cleaning all Docker resources...$(NC)"
	docker buildx prune -af
	docker system prune -af --volumes

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

benchmark: ## Run a quick benchmark
	@echo "$(GREEN)Running benchmark...$(NC)"
	docker run --rm $(REGISTRY)/mtgjson:$(TAG) python3 -c "\
import time; \
import mtgjson_rust; \
start = time.time(); \
for i in range(1000): \
    card = mtgjson_rust.MtgjsonCard(); \
print(f'Created 1000 cards in {time.time() - start:.4f}s')"

logs: ## View build logs
	docker buildx bake mtgjson --progress=plain

# Development helpers
install-buildx: ## Install Docker Buildx (if not available)
	@echo "$(GREEN)Installing Docker Buildx...$(NC)"
	docker buildx install

setup-builder: ## Setup multi-platform builder
	@echo "$(GREEN)Setting up multi-platform builder...$(NC)"
	docker buildx create --name mtgjson-builder --use --bootstrap
	docker buildx ls

# Quick development cycle
dev-cycle: build-local inspect ## Quick development cycle: build and test
	@echo "$(GREEN)Development cycle complete!$(NC)"

# Production release cycle
release: clean build test push ## Full release cycle: clean, build, test, push
	@echo "$(GREEN)Release cycle complete!$(NC)"