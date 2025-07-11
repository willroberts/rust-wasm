# rust-wasm

A demo of WebAssembly via Rust and `wasm-pack`.

## Compile WASM

Use a multi-stage Docker build to compile Rust to WASM, and place the resulting code in `build/`.

```bash
DOCKER_BUILDKIT=1 podman build -t rust-wasm . --output build/ # or `make build`
```

## Run Node.js server

There is a Node.js application in `build/www` which imports WASM from `build/`.

```bash
npm start --prefix build/www # or `make start`
```

Then, visit `http://localhost:8080` in a browser to execute WASM.
