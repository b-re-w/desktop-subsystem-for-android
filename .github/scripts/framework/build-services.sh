#!/usr/bin/env bash
# Builds services.jar with DSA's patches (the window manager, package manager and WebView
# service changes in dsa/guest/framework/patches/frameworks/base), in a tree fetch-source.py made
# from the services lock (framework_lock.py services-lock): every project the published build's
# manifest syncs by default, each at the manifest's commit.
#
#   build-services.sh <tree> <android16|android17> <out folder>
#
# The Java framework reaches across most of the platform, so unlike build.sh this builds in the
# whole tree, with the published build's own product (the lock's target) and release
# configuration, and so makes the services.jar that build made, with the patches. No dexpreopt
# (make-image.py compiles the jar itself against the guest's boot image, as it compiles the
# apps), so no boot image and no dex2oat.
#
# Only the dex jar is built, not the install path. Installing services.jar also installs what
# the module requires (its JNI libraries, and through them native and Rust code), which is
# thousands of compile steps and more disk than a standard runner has. So the build analyses the
# tree first (the goal `nothing`), asks ninja which file the install rule copies, and builds that
# file alone: the javac, R8 and d8 steps and the host tools they run. The jar goes to <out folder>
# under its target_files name (framework_lock.py java-outputs says where the build installs it).
#
# The swap Soong's analysis of the whole tree needs (DSA_ANALYSIS_SWAP, a swap file the workflow
# made) is given back to the disk once the analysis is done, as the Java build does not need it.
# DSA_BUILD_MINUTES (280 by default) bounds the whole script, so that a build that runs out of
# time says so before the job's limit stops it without a word. A failure says whether it was the
# disk, the memory or the time.
set -euo pipefail

top="$(cd "$1" && pwd)"
android="$2"
out="$3"
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../../.." && pwd)"
lock="$repo/dsa/guest/framework/framework-lock.json"
minutes="${DSA_BUILD_MINUTES:-280}"
deadline=$(( SECONDS + minutes * 60 ))

target="$(jq -r --arg a "$android" '.versions[] | select(.android == $a) | .target' "$lock")"
release="$(jq -r --arg a "$android" '.versions[] | select(.android == $a) | .release' "$lock")"
[[ -n "$target" && "$target" != null && -n "$release" && "$release" != null ]] \
  || { echo "the lock has no $android" >&2; exit 1; }
product="${target%-userdebug}"

room() {
  echo "::group::Disk and memory: $1"
  df -h / /mnt 2>/dev/null || df -h /
  free -h
  swapon --show || true
  echo "::endgroup::"
}

# Says why a build step failed: the disk, the memory, the time, or the build itself.
explain() {
  local status="$1" what="$2" avail logs
  avail="$(df -k --output=avail "$top" | tail -1 | tr -d ' ')"
  logs="$(cat out/error.log out/soong.log 2>/dev/null || true)"
  room "after the failed $what"
  if (( status == 124 || status == 137 && SECONDS >= deadline )); then
    echo "::error::$what ran out of time: it was stopped after the $minutes minutes the job gives it (DSA_BUILD_MINUTES)"
  elif (( avail < 2 * 1024 * 1024 )) || grep -q "No space left on device" <<< "$logs"; then
    echo "::error::$what ran out of disk: $(( avail / 1024 )) MiB left on the tree's disk"
  elif sudo dmesg 2>/dev/null | grep -qiE "out of memory|oom-kill|killed process" \
      || grep -qE "OutOfMemoryError|signal: killed|fatal error: runtime: out of memory" <<< "$logs"; then
    echo "::error::$what ran out of memory (the kernel's OOM killer or Java's heap; see free above)"
    sudo dmesg 2>/dev/null | grep -iE "out of memory|oom-kill|killed process" | tail -5 || true
  else
    echo "::error::$what failed (status $status), with disk and memory to spare; the errors are above"
  fi
  echo "::group::What the build could not find"
  grep -hoE "(depends on undefined module|missing dependencies?:|module source path|unrecognized module type|no such file or directory|Cannot find|not found)[^\"]{0,240}" \
    <<< "$logs" | sort | uniq -c | sort -rn | head -80 || true
  echo "::endgroup::"
  tail -n 120 out/error.log 2>/dev/null || true
  exit "$status"
}

# soong_ui, as m runs it, within what is left of the time.
build() {
  local what="$1" left status=0
  shift
  left=$(( deadline - SECONDS ))
  (( left > 60 )) || { explain 124 "$what"; }
  timeout --signal=TERM --kill-after=120 "$left" \
    build/soong/soong_ui.bash --make-mode -j"$(nproc)" "$@" || status=$?
  (( status == 0 )) || explain "$status" "$what"
}

# Every patch, the ones to Java sources included: the whole tree has every file they change.
uv run "$repo/dsa/guest/framework/apply-patches.py" --android "$android" --top "$top"

cd "$top"
export ALLOW_MISSING_DEPENDENCIES=true
export SKIP_ABI_CHECKS=true
export DISABLE_ARTIFACT_PATH_REQUIREMENTS=true
export BUILD_USERNAME=dsa-framework BUILD_HOSTNAME=github-actions
# make-image.py compiles the jar against the guest's own boot image; the build makes no odex.
export WITH_DEXPREOPT=false
# Android 17 runs siso by default; ninja is in build-tools.
export SOONG_NINJA=ninja

set +u
# shellcheck disable=SC1091
source build/envsetup.sh
lunch "$product-$release-userdebug"
set -u
product_out="$(get_build_var PRODUCT_OUT)"
room "before the analysis"

# The analysis of the whole tree, and the ninja files, with nothing built.
build "the analysis of the tree" nothing
room "after the analysis"

ninja_file="$(ls -t out/combined-*.ninja 2>/dev/null | head -1 || true)"
[[ -n "$ninja_file" ]] || ninja_file="out/soong/build.$TARGET_PRODUCT.ninja"
[[ -f "$ninja_file" ]] || { echo "::error::the analysis wrote no ninja file (looked for out/combined-*.ninja and $ninja_file)"; exit 1; }
ninja=prebuilts/build-tools/linux-x86/bin/ninja

# What each install rule copies: the module's own output, which the install path is a copy of.
outputs="$(uv run "$repo/dsa/guest/framework/framework_lock.py" java-outputs)"
names=()
jars=()
while read -r name installed; do
  query="$("$ninja" -f "$ninja_file" -t query "$product_out/$installed")" \
    || { echo "::error::ninja knows no rule for $product_out/$installed"; exit 1; }
  if ! jar="$(uv run "$repo/dsa/guest/framework/framework_lock.py" install-source <<< "$query")"; then
    head -40 <<< "$query"
    echo "::error::$installed: the build cannot make the module's own jar in place of the install path"
    exit 1
  fi
  echo "$installed is a copy of $jar"
  names+=("$name")
  jars+=("$jar")
done <<< "$outputs"

# The Java build needs no more than the runner's own memory and the first swap file.
if [[ -n "${DSA_ANALYSIS_SWAP:-}" && -f "$DSA_ANALYSIS_SWAP" ]]; then
  sudo swapoff "$DSA_ANALYSIS_SWAP"
  sudo rm -f "$DSA_ANALYSIS_SWAP"
  echo "gave the analysis swap ($DSA_ANALYSIS_SWAP) back to the disk"
fi
room "before the build"

echo "building ${jars[*]} for $product with release configuration $release"
build "the services.jar build" "${jars[@]}"
room "after the build"

mkdir -p "$out"
for i in "${!names[@]}"; do
  mkdir -p "$out/$(dirname "${names[$i]}")"
  cp "${jars[$i]}" "$out/${names[$i]}"
  ls -l "$out/${names[$i]}"
done
cp "$top/.dsa-patches.json" "$out/services-patches.json"
