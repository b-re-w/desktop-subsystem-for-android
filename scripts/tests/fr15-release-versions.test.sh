#!/usr/bin/env bash
# Checks the version rules of the release channels: a stable DSA release moves only
# major.minor with patch 0, a pre-release moves only the patch on the newest stable's
# major.minor, and a development build leads up to the next pre-release. The image lists
# installed stable computers read name stable images only, and image releases are never
# GitHub's latest release.
set -euo pipefail

root="$(cd "$(dirname "$0")/../.." && pwd)"
check="$root/.github/scripts/release/check-release-version.sh"
publish="$root/.github/scripts/release/publish-program-list.sh"
workflow="$root/.github/workflows/release-image.yml"
dev_build="$root/.github/workflows/dev-build.yml"

fail() {
    echo "fr15 release versions: $*" >&2
    exit 1
}

# What is published today: the pre-releases below 1.0 with their patches, the stable 1.0.0,
# the image releases and the lists' release.
published='v0.2.1
v0.2.2
v0.2.3
v1.0.0
android-17.0.0-16373615-14
android-17.0.0-16373615-15
images'

accepts() { bash "$check" check "$@" <<<"$published" >/dev/null 2>&1; }

# fr15_a_stable_release_moves_major_or_minor_with_patch_zero
accepts 1.1.0 stable || fail "the stable 1.1.0 after 1.0.0 was refused"
accepts 2.0.0 stable || fail "the stable 2.0.0 after 1.0.0 was refused"
! accepts 1.0.1 stable || fail "a stable release with patch 1 was accepted"
! accepts 1.0.0 stable || fail "a stable release equal to the newest stable was accepted"
! accepts 0.9.0 stable || fail "a stable release below the newest stable was accepted"
accepts 1.0.0 stable --replace || fail "replacing the stable 1.0.0 was refused"
why="$(bash "$check" check 1.0.1 stable <<<"$published" 2>&1 || true)"
[[ "$why" == *"patch 0"* ]] || fail "the refusal does not say why: $why"

# fr15_a_prerelease_moves_the_patch_above_the_newest_release
accepts 1.0.1 prerelease || fail "the pre-release 1.0.1 after 1.0.0 was refused"
accepts 1.0.5 prerelease || fail "a pre-release that skips patches was refused"
! accepts 1.1.0 prerelease || fail "a pre-release with patch 0 was accepted"
! accepts 1.1.1 prerelease || fail "a pre-release on another major.minor than the newest stable was accepted"
! accepts 0.2.4 prerelease || fail "a pre-release below the newest stable was accepted"
! accepts 1.0.1-dev.3 prerelease || fail "a development build was accepted as a release"
with_1_0_2="$published
v1.0.2"
! bash "$check" check 1.0.2 prerelease <<<"$with_1_0_2" >/dev/null 2>&1 \
    || fail "a pre-release equal to a published one was accepted"
! bash "$check" check 1.0.1 prerelease <<<"$with_1_0_2" >/dev/null 2>&1 \
    || fail "a pre-release below a published one on its major.minor was accepted"
bash "$check" check 1.0.2 prerelease --replace <<<"$with_1_0_2" >/dev/null \
    || fail "replacing the pre-release 1.0.2 was refused"
# Once 1.1.0 is stable, pre-releases are 1.1.z.
with_1_1="$with_1_0_2
v1.1.0"
bash "$check" check 1.1.1 prerelease <<<"$with_1_1" >/dev/null || fail "1.1.1 after the stable 1.1.0 was refused"
! bash "$check" check 1.0.3 prerelease <<<"$with_1_1" >/dev/null 2>&1 \
    || fail "a pre-release on the old major.minor was accepted after 1.1.0"

# The next pre-release, which a Development build leads up to.
[[ "$(bash "$check" next prerelease <<<"$published")" == 1.0.1 ]] || fail "the next pre-release after 1.0.0 is not 1.0.1"
[[ "$(bash "$check" next prerelease <<<"$with_1_0_2")" == 1.0.3 ]] || fail "the next pre-release after 1.0.2 is not 1.0.3"
[[ "$(bash "$check" next prerelease <<<"$with_1_1")" == 1.1.1 ]] || fail "the next pre-release after 1.1.0 is not 1.1.1"

# The workflow checks the version before anything is built, and the Development build names
# its installer <next pre-release>-dev.<run number>.
grep -Fq 'check-release-version.sh check "$VERSION" "$channel"' "$workflow" \
    || fail "the release workflow does not check the version rules"
grep -Fq 'check-release-version.sh next prerelease' "$dev_build" \
    || fail "the Development build does not lead up to the next pre-release"
grep -Fq 'installer_version=$NEXT-dev.${{ github.run_number }}' "$dev_build" \
    || fail "the Development build's installer is not <next pre-release>-dev.<run number>"

# fr15_the_stable_image_list_names_stable_images_only
grep -Fq -- '--next-revision --channel "$IMAGE_CHANNEL"' "$workflow" \
    || fail "the image revision does not follow the channel"
grep -Fq 'flags=(--latest=false)' "$workflow" || fail "an image release may become GitHub's latest release"
grep -Fq '[[ "$IMAGE_CHANNEL" == prerelease ]] && flags+=(--prerelease)' "$workflow" \
    || fail "a pre-release image is not published as a GitHub pre-release"
grep -Fq 'lists=(images-prerelease.json images-prerelease-arm64.json) ;;' "$workflow" \
    || fail "a pre-release does not publish the pre-release image lists alone"
grep -Fq 'lists=(images.json images-arm64.json images-prerelease.json images-prerelease-arm64.json) ;;' "$workflow" \
    || fail "a stable release does not publish all four image lists"
grep -Fq 'lists=("images-prerelease$suffix.json")' "$workflow" \
    || fail "a pre-release writes a list other than the pre-release one"
grep -Fq 'would name images that are not stable' "$workflow" \
    || fail "nothing stops a stable list from naming a pre-release image"
grep -Fq -- '--target "$GITHUB_SHA" --latest=false' "$workflow" \
    || fail "the lists' release may become GitHub's latest release"

# fr15_the_stable_program_list_names_stable_versions_only, with a gh that fails if called:
# the version is refused before anything is uploaded.
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
mkdir -p "$work/bin"
printf '#!/usr/bin/env bash\necho "gh was called: $*" >&2\nexit 1\n' > "$work/bin/gh"
chmod +x "$work/bin/gh"
list() {
    mkdir -p "$work/$1"
    printf '{\n  "format": 1,\n  "version": "%s",\n  "setup": {}\n}\n' "$2" > "$work/$1/program.json"
    echo "signature of $2" > "$work/$1/program.json.sig"
}
refused() { ! PATH="$work/bin:$PATH" bash "$publish" "$1" x64 "$work/$2" >/dev/null 2>&1; }
list pre 1.0.1
refused stable pre || fail "the pre-release 1.0.1 was put on the stable program list"
list dev 1.0.1-dev.4
refused stable dev || fail "a development build was put on the stable program list"
refused prerelease dev || fail "a development build was put on the pre-release program list"
why="$(PATH="$work/bin:$PATH" bash "$publish" stable x64 "$work/pre" 2>&1 || true)"
[[ "$why" == *"stable program list names only X.Y.0"* ]] || fail "the refusal does not say why: $why"
# A stable X.Y.0 gets as far as uploading, which the stand-in gh refuses.
list stable 1.1.0
why="$(PATH="$work/bin:$PATH" bash "$publish" stable x64 "$work/stable" 2>&1 || true)"
[[ "$why" == *"gh was called"* ]] || fail "the stable 1.1.0 was refused before publishing: $why"

echo "fr15 release versions: ok"
