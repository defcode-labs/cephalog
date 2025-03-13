# ClickHouse Docker Configuration
CLICKHOUSE_CONTAINER=clickhouse_test
CLICKHOUSE_PORT=8123
CLICKHOUSE_DB=test_db
CLICKHOUSE_USER=default
CLICKHOUSE_PASSWORD=""
CLICKHOUSE_HOST=localhost
CLICKHOUSE_IMAGE=clickhouse/clickhouse-server:latest
# CLICKHOUSE_URL?=http://$(CLICKHOUSE_USER)@$(CLICKHOUSE_HOST):$(CLICKHOUSE_PORT)
CLICKHOUSE_URL=http://localhost:8123

export CLICKHOUSE_URL=http://localhost:8123
export CLICKHOUSE_DB=test_db
export CLICKHOUSE_USER=default
# Start ClickHouse container for testing
.PHONY: up
up:
	docker run -d --rm \
		--name $(CLICKHOUSE_CONTAINER) \
		-p $(CLICKHOUSE_PORT):8123 \
		-e CLICKHOUSE_DB=$(CLICKHOUSE_DB) \
		-e CLICKHOUSE_DEFAULT_ACCESS_MANAGEMENT=1 \
		$(CLICKHOUSE_IMAGE)

# Wait for ClickHouse to be ready
.PHONY: wait
wait:
	@echo "Waiting for ClickHouse to be ready..."
	@until docker exec $(CLICKHOUSE_CONTAINER) clickhouse-client --query "SELECT 1" 2>/dev/null; do sleep 1; done
	@echo "ClickHouse is ready!"

# Run migrations (create test tables)
.PHONY: migrate
migrate: wait
	@echo "Running migrations..."
	@docker exec -i $(CLICKHOUSE_CONTAINER) clickhouse-client --query \
	"CREATE DATABASE IF NOT EXISTS $(CLICKHOUSE_DB);"

	@docker exec -i $(CLICKHOUSE_CONTAINER) clickhouse-client --query \
	"CREATE TABLE IF NOT EXISTS $(CLICKHOUSE_DB).logs ( \
	    id UUID DEFAULT generateUUIDv4(), \
	    timestamp DateTime DEFAULT now(), \
	    source_ip String, \
	    event_type LowCardinality(String), \
	    targeted_service String, \
	    targeted_endpoint String, \
	    request String, \
	    status String, \
	    action_taken String, \
	    threat_level String \
	) ENGINE = MergeTree() \
	ORDER BY (timestamp, source_ip, event_type) \
	PARTITION BY toYYYYMM(timestamp) \
	TTL timestamp + INTERVAL 90 DAY;"

	@echo "Migrations completed!"

# Run integration tests
.PHONY: test
test: migrate
	CARGO_ENV="CLICKHOUSE_URL=$(CLICKHOUSE_URL) CLICKHOUSE_DB=$(CLICKHOUSE_DB) CLICKHOUSE_USER=$(CLICKHOUSE_USER)" cargo test --test '*' -- --nocapture

# Stop and remove the container
.PHONY: down
down:
	docker stop $(CLICKHOUSE_CONTAINER)
	

# Restart ClickHouse
.PHONY: restart
restart: down up wait migrate
