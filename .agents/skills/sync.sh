#!/usr/bin/env bash
# Re-vendor the upstream skills at a pinned commit.
#
#   .agents/skills/sync.sh          copy from the pinned commit, report what changed
#   .agents/skills/sync.sh --bump   move the pin to the latest upstream main, then copy
#
# Vendored files stay byte-identical to upstream. A skill that needs changing for
# coding-lanes becomes ours, under docs/skills/.

set -euo pipefail

REPO="https://github.com/mattpocock/skills"
PIN_FILE="$(dirname "$0")/UPSTREAM.md"

# Skills to vendor: <name>:<upstream path relative to the repo root>
SKILLS=(
  "codebase-design:skills/engineering/codebase-design"
  "diagnosing-bugs:skills/engineering/diagnosing-bugs"
  "domain-modeling:skills/engineering/domain-modeling"
  "writing-for-agents:skills/productivity/writing-for-agents"
)

DEST="$(cd "$(dirname "$0")" && pwd)"

pinned_commit() {
  grep -oE '`[0-9a-f]{40}`' "$PIN_FILE" | head -1 | tr -d '`'
}

fetch() {
  local commit="$1" tmp
  tmp="$(mktemp -d)"
  echo "fetching $REPO @ $commit" >&2
  git clone --quiet "$REPO" "$tmp/repo"
  git -C "$tmp/repo" checkout --quiet "$commit"
  echo "$tmp/repo"
}

copy_skill() {
  local src_root="$1" name="$2" up_path="$3"
  local src="$src_root/$up_path"
  local dst="$DEST/$name"

  mkdir -p "$dst"
  # Copy every file except the harness-specific agents/ directory.
  ( cd "$src" && find . -type f -not -path "./agents/*" -print0 ) |
    while IFS= read -r -d '' f; do
      mkdir -p "$dst/$(dirname "$f")"
      cp "$src/$f" "$dst/$f"
    done
}

report_diff() {
  local name="$1" up_path="$2" src_root="$3"
  local src="$src_root/$up_path" dst="$DEST/$name"
  if ! diff -rq "$src" "$dst" --exclude agents >/dev/null 2>&1; then
    echo "  $name differs from upstream:"
    diff -r "$src" "$dst" --exclude agents 2>/dev/null | grep "^[<>]" | head -20 || true
  fi
}

main() {
  local commit
  if [ "${1:-}" = "--bump" ]; then
    commit="$(git ls-remote "$REPO" refs/heads/main | cut -f1)"
    echo "bumping pin to $commit"
    sed -i '' -E "s/\`[0-9a-f]{40}\`/\`$commit\`/" "$PIN_FILE" 2>/dev/null ||
      sed -i -E "s/\`[0-9a-f]{40}\`/\`$commit\`/" "$PIN_FILE"
  else
    commit="$(pinned_commit)"
  fi

  local tmp
  tmp="$(fetch "$commit")"

  for entry in "${SKILLS[@]}"; do
    local name="${entry%%:*}" up_path="${entry#*:}"
    copy_skill "$tmp" "$name" "$up_path"
    report_diff "$name" "$up_path" "$tmp"
  done

  cp "$tmp/LICENSE" "$DEST/LICENSE"

  rm -rf "$tmp"
  echo "done — provenance in .agents/skills/UPSTREAM.md"
}

main "$@"
