# shellcheck shell=bash
# Load repo-root `.env` into the current shell (no-op if missing).
# Existing environment variables win (CI/export beats `.env`).
#
# Usage from other scripts:
#   # shellcheck source=/dev/null
#   source "$(cd "$(dirname "$0")/.." && pwd)/scripts/load-dotenv.sh"

_uie_dotenv_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
_uie_dotenv_file="${UIE_DOTENV_PATH:-$_uie_dotenv_root/.env}"

if [[ -f "$_uie_dotenv_file" ]]; then
  while IFS= read -r _uie_line || [[ -n "${_uie_line:-}" ]]; do
    # Strip CR (Windows) and trim leading whitespace.
    _uie_line="${_uie_line%$'\r'}"
    _uie_line="${_uie_line#"${_uie_line%%[![:space:]]*}"}"
    [[ -z "$_uie_line" || "$_uie_line" == \#* ]] && continue
    [[ "$_uie_line" != *=* ]] && continue
    _uie_key="${_uie_line%%=*}"
    _uie_val="${_uie_line#*=}"
    _uie_key="${_uie_key%"${_uie_key##*[![:space:]]}"}"
    # Skip if already set in the environment.
    if [[ -n "${_uie_key}" && -z "${!_uie_key+x}" ]]; then
      # Expand $HOME / $PWD etc. the way `source` would.
      _uie_val="$(eval "printf '%s' \"$_uie_val\"")"
      export "${_uie_key}=${_uie_val}"
    fi
  done < "$_uie_dotenv_file"
fi

unset _uie_dotenv_root _uie_dotenv_file _uie_line _uie_key _uie_val
