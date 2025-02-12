.PHONY: all dev prod clean setup install uninstall

# Environment variables
USER ?= $(shell whoami)
GROUP ?= $(shell id -gn)

# Directories
PROD_BASE_DIR = /opt/kewr-panel
PROD_WWW_DIR = /var/lib/kewr-panel
PROD_LOG_DIR = /var/log/kewr-panel
PROD_CONFIG_DIR = /etc/kewr-panel
PROD_RUNTIME_DIR = /var/run/kewr-panel

DEV_BASE_DIR = ./
DEV_WWW_DIR = ./www
DEV_LOG_DIR = ./logs
DEV_CONFIG_DIR = ./config
DEV_RUNTIME_DIR = ./run

# Default target
all: dev

# Development build
dev:
	cargo build

# Production build
prod:
	cargo build --release

# Clean build artifacts
clean:
	cargo clean
	rm -rf $(DEV_LOG_DIR)/*
	rm -rf $(DEV_RUNTIME_DIR)/*

# Setup development environment
setup:
	mkdir -p $(DEV_WWW_DIR) $(DEV_LOG_DIR) $(DEV_CONFIG_DIR) $(DEV_RUNTIME_DIR)
	[ -f .env ] || cp .env.example .env

# Install for production
install: prod
	sudo mkdir -p $(PROD_BASE_DIR) $(PROD_WWW_DIR) $(PROD_LOG_DIR) $(PROD_CONFIG_DIR) $(PROD_RUNTIME_DIR)
	sudo chown -R $(USER):$(GROUP) $(PROD_BASE_DIR) $(PROD_WWW_DIR) $(PROD_LOG_DIR) $(PROD_CONFIG_DIR) $(PROD_RUNTIME_DIR)
	sudo cp target/release/panel $(PROD_BASE_DIR)/
	[ -f $(PROD_CONFIG_DIR)/.env ] || sudo cp .env.example $(PROD_CONFIG_DIR)/.env

# Create and enable systemd service (Linux only)
service:
	sudo tee /etc/systemd/system/kewr-panel.service > /dev/null << EOF
[Unit]
Description=Kewr Panel System Monitor
After=network.target

[Service]
Type=simple
User=$(USER)
Group=$(GROUP)
ExecStart=$(PROD_BASE_DIR)/panel
Restart=on-failure
WorkingDirectory=$(PROD_BASE_DIR)

[Install]
WantedBy=multi-user.target
EOF
	sudo systemctl daemon-reload
	sudo systemctl enable kewr-panel
	sudo systemctl start kewr-panel

# Uninstall
uninstall:
	-sudo systemctl stop kewr-panel
	-sudo systemctl disable kewr-panel
	-sudo rm /etc/systemd/system/kewr-panel.service
	-sudo systemctl daemon-reload
	-sudo rm -rf $(PROD_BASE_DIR) $(PROD_WWW_DIR) $(PROD_LOG_DIR) $(PROD_CONFIG_DIR) $(PROD_RUNTIME_DIR)