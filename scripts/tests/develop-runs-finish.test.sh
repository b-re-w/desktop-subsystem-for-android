#!/usr/bin/env bash
# Checks that runs do not pile up and that no newer run cancels one in progress: every
# workflow has one concurrency group per workflow and branch (or a fixed one), never one per
# run or per commit, so at most one run goes and the newest waits, and none cancels a run
# that has started. The one exception is a pull request run that waits for the owner's
# approval, whose approval job keeps only the newest waiting run instead.
set -euo pipefail

root="$(cd "$(dirname "$0")/../.." && pwd)"
for workflow in "$root"/.github/workflows/*.yml; do
  name="$(basename "$workflow")"
  # The top-level concurrency block, without comments.
  block="$(awk '{ sub(/\r$/, "") } /^concurrency:/ { on = 1; next } on && /^[^ #]/ { exit }
    on && !/^ *#/ { print }' "$workflow")"
  group="$(grep -E '^  group: ' <<<"$block" || true)"
  if [[ -z "$group" ]]; then
    echo "$name: no concurrency group, so its runs pile up" >&2
    exit 1
  fi
  # A pull request run may have a group of its own only when it waits in an "approval" job
  # on the run-tests environment whose own group, one per pull request, cancels the older
  # run still waiting: a run waiting for approval counts as in progress, so a shared group
  # would hold every newer run behind it. An approved run has passed that job, so nothing
  # cancels it.
  per_pr_run="github.event_name == 'pull_request' && format('run-{0}', github.run_id) || "
  if grep -Fq "$per_pr_run" <<<"$group"; then
    if ! grep -Fq "environment: \${{ github.event_name == 'pull_request' && 'run-tests' || '' }}" "$workflow" ||
      ! grep -Fq "github.event_name == 'pull_request' && github.event.pull_request.number || github.run_id }}" "$workflow" ||
      ! grep -Fq "cancel-in-progress: \${{ github.event_name == 'pull_request' }}" "$workflow"; then
      echo "$name: a pull request run has its own group but no approval job replaces the older waiting run" >&2
      exit 1
    fi
    group="${group/"$per_pr_run"/}"
  fi
  if grep -Eq 'github\.(sha|run_id|run_number|run_attempt)' <<<"$group"; then
    echo "$name: the concurrency group is per run or per commit, so its runs pile up" >&2
    exit 1
  fi
  if grep -Fq 'github.' <<<"$group" && ! grep -Fq 'github.ref }}' <<<"$group"; then
    echo "$name: the concurrency group does not name the branch" >&2
    exit 1
  fi
  if ! grep -Fxq '  cancel-in-progress: false' <<<"$block"; then
    echo "$name: a newer run can cancel the run in progress" >&2
    exit 1
  fi
done
echo "runs finish: ok"
