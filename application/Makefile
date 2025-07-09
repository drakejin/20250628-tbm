# TBM Application Makefile
# Modern Rust server API development

.PHONY: help build run dev test clean fmt clippy check install docker-build docker-run docs

# Default target
help: ## Show this help message
	@echo "TBM Application - Available commands:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

# Development commands
install: ## Install dependencies
	cargo fetch

build: ## Build the application
	cargo build

build-release: ## Build the application in release mode
	cargo build --release

run: ## Run the application in development mode
	RUST_LOG=info cargo run

dev: ## Run the application with auto-reload (requires cargo-watch)
	cargo watch -x run

test: ## Run all tests
	cargo test

test-verbose: ## Run tests with verbose output
	cargo test -- --nocapture

# Code quality commands
fmt: ## Format code
	cargo fmt

fmt-check: ## Check code formatting
	cargo fmt -- --check

clippy: ## Run clippy linter
	cargo clippy -- -D warnings

clippy-fix: ## Run clippy with automatic fixes
	cargo clippy --fix

check: ## Run cargo check
	cargo check

audit: ## Run security audit
	cargo audit

# Combined quality check
quality: fmt-check clippy test ## Run all quality checks

# Clean commands
clean: ## Clean build artifacts
	cargo clean

clean-all: clean ## Clean all artifacts including target directory
	rm -rf target/

# Documentation
docs: ## Generate and open documentation
	cargo doc --open

docs-build: ## Build documentation
	cargo doc --no-deps

# Docker commands
docker-build: ## Build Docker image
	docker build -t tbm-application .

docker-run: ## Run Docker container
	docker run -p 3000:3000 tbm-application

docker-compose-up: ## Start services with docker-compose
	docker-compose up -d

docker-compose-down: ## Stop services with docker-compose
	docker-compose down

# Environment setup
setup: ## Setup development environment
	@echo "Setting up development environment..."
	rustup update
	rustup component add rustfmt clippy
	cargo install cargo-watch cargo-audit
	@echo "Development environment setup complete!"

# Health check
health: ## Check application health (requires running server)
	curl -s http://localhost:3000/health | jq .

# Swagger UI
swagger: ## Open Swagger UI (requires running server)
	open http://localhost:3000/swagger-ui

# Performance and profiling
bench: ## Run benchmarks
	cargo bench

profile: ## Run with profiling
	RUST_LOG=info cargo run --release

# Database commands (for future use)
db-migrate: ## Run database migrations (placeholder)
	@echo "Database migrations not implemented yet"

db-reset: ## Reset database (placeholder)
	@echo "Database reset not implemented yet"

# Deployment commands
deploy-staging: ## Deploy to staging (placeholder)
	@echo "Staging deployment not implemented yet"

deploy-prod: ## Deploy to production (placeholder)
	@echo "Production deployment not implemented yet"

# Development workflow
dev-setup: setup install ## Complete development setup
	@echo "Development setup complete! Run 'make dev' to start development server."

quick-check: fmt clippy test ## Quick development check

full-check: clean build test clippy fmt-check audit ## Full project check

# Server management
start: run ## Alias for run

stop: ## Stop running processes (manual)
	@echo "Please manually stop the running server (Ctrl+C)"

restart: stop start ## Restart the server

# Logs
logs: ## Show application logs (requires running server)
	@echo "Application logs are displayed in the terminal where 'make run' or 'make dev' is executed"

# Version info
version: ## Show version information
	@echo "TBM Application version: $(shell cargo pkgid | cut -d# -f2)"
	@echo "Rust version: $(shell rustc --version)"
	@echo "Cargo version: $(shell cargo --version)"
