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
# configuration, and so makes the services.jar that build made, with the patches. Only the jar is
# built: no dexpreopt (make-image.py compiles the jar itself against the guest's boot image, as it
# compiles the apps), so no boot image and no dex2oat. The jar goes to <out folder> under its
# target_files name (framework_lock.py java-outputs says where the build installs it).
set -euo pipefail

top="$(cd "$1" && pwd)"
android="$2"
out="$3"
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../../.." && pwd)"
lock="$repo/dsa/guest/framework/framework-lock.json"

target="$(jq -r --arg a "$android" '.versions[] | select(.android == $a) | .target' "$lock")"
release="$(jq -r --arg a "$android" '.versions[] | select(.android == $a) | .release' "$lock")"
[[ -n "$target" && "$target" != null && -n "$release" && "$release" != null ]] \
  || { echo "the lock has no $android" >&2; exit 1; }
product="${target%-userdebug}"

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

outputs="$(uv run "$repo/dsa/guest/framework/framework_lock.py" java-outputs)"
targets=()
while read -r _ installed; do
  targets+=("$product_out/$installed")
done <<< "$outputs"
echo "building ${targets[*]} for $product with release configuration $release"

status=0
m -j"$(nproc)" "${targets[@]}" || status=$?
if (( status != 0 )); then
  echo "::group::What the build could not find"
  for log in out/error.log out/soong.log; do
    [[ -f "$log" ]] || continue
    grep -hoE "(depends on undefined module|missing dependencies?:|module source path|unrecognized module type|no such file or directory|Cannot find|not found)[^\"]{0,240}" "$log" \
      | sort | uniq -c | sort -rn | head -80 || true
  done
  echo "::endgroup::"
  tail -n 120 out/error.log 2>/dev/null || true
  exit "$status"
fi

mkdir -p "$out"
while read -r name installed; do
  mkdir -p "$out/$(dirname "$name")"
  cp "$product_out/$installed" "$out/$name"
  ls -l "$out/$name"
done <<< "$outputs"
cp "$top/.dsa-patches.json" "$out/services-patches.json"
