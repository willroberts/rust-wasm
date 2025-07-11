.PHONY: build
build:
	DOCKER_BUILDKIT=1 podman build -t rust-wasm . --output build/

.PHONY: test
test:
	cargo test

.PHONY: start
start:
	@echo "Starting Node.js server; visit http://localhost:8080 to test."
	npm install --prefix build/www
	npm start --prefix build/www
