.PHONY: build
build:
	DOCKER_BUILDKIT=1 podman build -t rust-wasm . --output build/
