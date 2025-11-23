#!/usr/bin/env bash
set -uo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LOG_DIR="${ROOT}/tmp/cargo-make-target-logs"
mkdir -p "${LOG_DIR}"

if ! command -v cargo-make >/dev/null 2>&1; then
  echo "cargo-make not installed (install via: cargo install cargo-make)" >&2
  exit 1
fi

mapfile -t TARGETS < <(
  cd "${ROOT}" && cargo make --list-all-steps \
    | grep -Eo '^[^-[:space:]]* -' \
    | cut -d ' ' -f1 \
    | sed '/^$/d' \
    | sort -u
)

pass=0
fail=0

for t in "${TARGETS[@]}"; do
  log="${LOG_DIR}/${t}.log"
  printf '[RUN ] %s ... ' "${t}"
  (cd "${ROOT}" && cargo make "${t}" >"${log}" 2>&1)
  status=$?
  if [ ${status} -eq 0 ]; then
    printf 'PASS\n'
    pass=$((pass + 1))
  else
    printf 'FAIL (see %s)\n' "${log}"
    fail=$((fail + 1))
  fi
done

echo "Summary: ${pass} passed, ${fail} failed. Logs -> ${LOG_DIR}"

if [ ${fail} -ne 0 ]; then
  exit 1
fi
