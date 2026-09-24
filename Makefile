# Define variables
COMPOSE = docker compose

# Default target
all: build run

# Build the Docker containers
build:
	$(COMPOSE) -f docker-compose-db.yml build
	$(COMPOSE) -f docker-compose-rs.yml build

# Run the Docker containers
run:
	$(COMPOSE) -f docker-compose-db.yml up -d
	$(COMPOSE) -f docker-compose-rs.yml up -d

# Stop the Docker containers
stop:
	$(COMPOSE) -f docker-compose-rs.yml down
	$(COMPOSE) -f docker-compose-db.yml down

# Clean up Docker resources
clean:
	$(COMPOSE) -f docker-compose-rs.yml down -v
	$(COMPOSE) -f docker-compose-db.yml down -v
	$(COMPOSE) -f docker-compose-rs.yml rm -f
	$(COMPOSE) -f docker-compose-db.yml rm -f
	$(COMPOSE) -f docker-compose-rs.yml rmi -f
	$(COMPOSE) -f docker-compose-db.yml rmi -f

# Phony targets
.PHONY: all build run stop clean