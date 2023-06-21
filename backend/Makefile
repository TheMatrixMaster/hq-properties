NAME=hq-properties
VERSION=$(shell git rev-parse HEAD)
SEMVER_VERSION=$(shell grep version Cargo.toml | awk -F"\"" '{print $$2}' | head -n 1)
REPO=thematrixmaster
WATCH_DIR=.watchdir
SHELL := /bin/bash

has_secrets:
	@[[ $$POSTGRES_DB ]] || (echo "source env.sh first"; exit 2)

no_postgres:
	@[ -z "$$(docker ps -q -f ancestor="postgres:15.3-alpine")" ] || (echo "db running"; exit 2)
has_postgres:
	@[ -n "$$(docker ps -q -f ancestor="postgres:15.3-alpine")" ] || (echo "db not running"; exit 2)

db: has_secrets	no_postgres
	@echo "Starting postgres container"
	docker run --rm -d \
		-p "5432:5432" \
		--expose 5432 \
		-e POSTGRES_DB="$$POSTGRES_DB" \
		-e POSTGRES_PASSWORD="$$POSTGRES_PASSWORD" \
		-e POSTGRES_USER="$$POSTGRES_USER" \
		-it postgres:15.3-alpine

reset_db: has_secrets has_postgres
	@echo "Clearing the postgres database"
	diesel database reset

setup:
	cargo install diesel_cli --no-default-features --features postgres
	rustup override set $$(cat .rustup)

test: has_secrets has_postgres
	cargo test --features build-api --bin api

run_api: has_secrets has_postgres
	cargo run --features build-api --bin api

run_fw: has_secrets has_postgres
	cargo run --features build-fw --bin fw ${WATCH_DIR}
