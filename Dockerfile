FROM alpine AS builder

RUN apk add --no-cache npm curl build-base
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=/root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

WORKDIR /wasm
COPY . /wasm

RUN wasm-pack build

FROM scratch AS output
COPY --from=builder /wasm/pkg/package.json /
COPY --from=builder /wasm/pkg/rust_wasm_bg.js /
COPY --from=builder /wasm/pkg/rust_wasm_bg.wasm /
COPY --from=builder /wasm/pkg/rust_wasm_bg.wasm.d.ts /
COPY --from=builder /wasm/pkg/rust_wasm.d.ts /
COPY --from=builder /wasm/pkg/rust_wasm.js /
