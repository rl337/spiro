#!/usr/bin/env bash
# US-030: build/run a Vulkan uie-llm container on crystalline (Pi 5).
# Does not touch hn_laya. Uses home_network inventory for SSH.
#
# Usage:
#   ./scripts/crystalline-pi-gpu-experiment.sh sync-model   # scp small GGUF once
#   ./scripts/crystalline-pi-gpu-experiment.sh build
#   ./scripts/crystalline-pi-gpu-experiment.sh list-gpu
#   ./scripts/crystalline-pi-gpu-experiment.sh prompt "hello"
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
HN_ROOT="${HN_ROOT:-$(cd "$ROOT/../../home_network" 2>/dev/null && pwd || true)}"
if [[ -z "${HN_ROOT}" || ! -f "${HN_ROOT}/inventory.env" ]]; then
  HN_ROOT="$(cd "$ROOT/../home_network" 2>/dev/null && pwd || true)"
fi
if [[ -z "${HN_ROOT}" || ! -f "${HN_ROOT}/inventory.env" ]]; then
  echo "error: set HN_ROOT to the home_network checkout (need inventory.env)" >&2
  exit 1
fi

# shellcheck disable=SC1091
set -a
source "${HN_ROOT}/inventory.env"
set +a

HOST="${CRYSTALLINE_HOST:?CRYSTALLINE_HOST missing in inventory.env}"
USER_="${REMOTE_USER:-rlee}"
TARGET="${USER_}@${HOST}"
REMOTE_BUILD="${CRYSTALLINE_UIE_BUILD_DIR:-/var/lib/home_network/build/uie-llm}"
REMOTE_MODELS="${CRYSTALLINE_UIE_MODELS:-/mnt/bfd/home_network/volumes/uie-gguf}"
IMAGE="${CRYSTALLINE_UIE_IMAGE:-uie-llm:vulkan-pi}"
LOCAL_GGUF="${CRYSTALLINE_LOCAL_GGUF:-$HOME/.lmstudio/models/Qwen/Qwen3-0.6B-GGUF/Qwen3-0.6B-Q8_0.gguf}"

ssh_c() {
  ssh -o BatchMode=yes -o ConnectTimeout=10 "$TARGET" "$@"
}

docker_gpu() {
  local render_gid video_gid
  render_gid="$(ssh_c 'getent group render | cut -d: -f3')"
  video_gid="$(ssh_c 'getent group video | cut -d: -f3')"
  ssh_c "docker run --rm \
    --device /dev/dri \
    --group-add ${render_gid} \
    --group-add ${video_gid} \
    -v $(printf %q "$REMOTE_MODELS"):/models:ro \
    -e UIE_GGUF_MODEL=/models/model.gguf \
    $(printf %q "$IMAGE") \
    $*"
}

cmd="${1:-}"
shift || true

case "$cmd" in
  sync-model)
    [[ -f "$LOCAL_GGUF" ]] || {
      echo "error: missing GGUF at $LOCAL_GGUF" >&2
      exit 1
    }
    ssh_c "mkdir -p $(printf %q "$REMOTE_MODELS")"
    echo "scp → ${TARGET}:${REMOTE_MODELS}/model.gguf" >&2
    scp -o BatchMode=yes "$LOCAL_GGUF" "${TARGET}:${REMOTE_MODELS}/model.gguf"
    ssh_c "ls -lh $(printf %q "$REMOTE_MODELS")/model.gguf"
    ;;
  build)
    ssh_c "mkdir -p $(printf %q "$REMOTE_BUILD")"
    echo "rsync repo → ${TARGET}:${REMOTE_BUILD}" >&2
    rsync -az --delete \
      -e "ssh -o BatchMode=yes" \
      --exclude '.git' \
      --exclude 'target' \
      --exclude 'web/*/node_modules' \
      --exclude 'web/*/playwright-report' \
      --exclude '.env' \
      "$ROOT/" "${TARGET}:${REMOTE_BUILD}/"
    echo "docker build ${IMAGE} on crystalline (long: llama Vulkan + rustc)" >&2
    ssh_c "cd $(printf %q "$REMOTE_BUILD") && docker build \
      -f deploy/crystalline-uie-llm/Dockerfile \
      -t $(printf %q "$IMAGE") \
      ."
    ;;
  list-gpu)
    docker_gpu list-gpu
    ;;
  prompt)
    prompt="${1:?usage: $0 prompt \"text\"}"
    docker_gpu --prompt "$(printf %q "$prompt")" --max-new 32
    ;;
  cpu-prompt)
    prompt="${1:?usage: $0 cpu-prompt \"text\"}"
    docker_gpu --prompt "$(printf %q "$prompt")" --max-new 32 --cpu-only
    ;;
  *)
    cat >&2 <<EOF
usage: $0 {sync-model|build|list-gpu|prompt|cpu-prompt} ...
HN_ROOT=$HN_ROOT
TARGET=$TARGET
IMAGE=$IMAGE
EOF
    exit 2
    ;;
esac
