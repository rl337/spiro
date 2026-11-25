# US-030 experiment image for crystalline (Raspberry Pi 5).
# Host: leave hn_laya alone; run this as a separate container with /dev/dri.
#
# Build on-node (aarch64): see scripts/crystalline-pi-gpu-experiment.sh

FROM rust:1-bookworm AS build

RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      build-essential \
      cmake \
      pkg-config \
      clang \
      libclang-dev \
      libvulkan-dev \
      glslang-tools \
      glslc \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /src
# Full workspace context is rsynced by crystalline-pi-gpu-experiment.sh
COPY . .

# Low parallelism — crystalline has ~4GB free under Swarm + laya.
ENV CARGO_BUILD_JOBS=2 \
    CARGO_TERM_COLOR=always
RUN cargo build -p uie-llm --release --features vulkan \
 && cargo build -p uie-loader-gguf --release --features vulkan --example list_ggml_devices

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      ca-certificates \
      libvulkan1 \
      mesa-vulkan-drivers \
      vulkan-tools \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=build /src/target/release/uie-llm /usr/local/bin/uie-llm
COPY --from=build /src/target/release/examples/list_ggml_devices /usr/local/bin/list_ggml_devices

ENV UIE_GGUF_MODEL=/models/model.gguf
ENTRYPOINT ["/usr/local/bin/uie-llm"]
CMD ["list-gpu"]
