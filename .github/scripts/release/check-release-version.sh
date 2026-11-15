#!/usr/bin/env bash
# Usage:
#   .github/scripts/release/check-release-version.sh check <version> <stable|prerelease> [--replace]
#       checks that an installer release of <version> on that channel follows the version
#       rules, given the release tags already published, one per line on standard input
#       (gh api repos/<repo>/releases --jq '.[].tag_name'). With --replace the release of
#       <version> itself is being replaced, so it does not count as published.
#   .github/scripts/release/check-release-version.sh next prerelease
#       prints the version the next pre-release gets, given the published tags on standard
#       input. The Development build names its installer <that version>-dev.<run number>.
#
# The rules, which DSA's update channels rely on:
#
#   stable       moves only major.minor, and its patch is 0: after v1.0.0 the next stable is
#                v1.1.0 or v2.0.0. Its major.minor is above the newest stable release's.
#   prerelease   moves only the patch, which is above 0: after v1.0.0 come v1.0.1, v1.0.2, ...
#                until the next stable v1.1.0. Its major.minor is the newest stable release's,
#                and its patch is above that of every release on that major.minor. So every
#                pre-release stays below the next stable release.
#   development  <major>.<minor>.<patch>-dev.<run number>, never published.
#
# A published release is stable when it is 1.0 or above with patch 0; every release below 1.0
# was a pre-release, numbered before these rules. Tags that are not v<major>.<minor>.<patch>
# (the image releases android-<version>, the lists' release "images") are not counted.

set -euo pipefail

die() {
    echo "check-release-version: $*" >&2
    exit 1
}

usage="usage: check-release-version.sh check <version> <stable|prerelease> [--replace] | next prerelease"

# The published releases as "major minor patch", one per line.
published=()
read_published() {
    local tag
    while IFS= read -r tag; do
        tag="${tag%$'\r'}"
        if [[ "$tag" =~ ^v([0-9]+)\.([0-9]+)\.([0-9]+)$ ]]; then
            published+=("$((10#${BASH_REMATCH[1]})) $((10#${BASH_REMATCH[2]})) $((10#${BASH_REMATCH[3]}))")
        fi
    done
}

# The newest stable release as "major minor", or nothing when none is published.
newest_stable() {
    local entry major minor patch best_major=-1 best_minor=-1
    for entry in "${published[@]+"${published[@]}"}"; do
        read -r major minor patch <<<"$entry"
        (( major >= 1 && patch == 0 )) || continue
        if (( major > best_major || (major == best_major && minor > best_minor) )); then
            best_major=$major best_minor=$minor
        fi
    done
    (( best_major >= 0 )) && echo "$best_major $best_minor"
    return 0
}

# The highest patch published on major.minor $1.$2, or -1 when none is.
highest_patch() {
    local entry major minor patch best=-1
    for entry in "${published[@]+"${published[@]}"}"; do
        read -r major minor patch <<<"$entry"
        if (( major == $1 && minor == $2 && patch > best )); then
            best=$patch
        fi
    done
    echo "$best"
}

# The newest release of any kind as "major minor", or nothing when none is published.
newest_any() {
    local entry major minor patch best_major=-1 best_minor=-1
    for entry in "${published[@]+"${published[@]}"}"; do
        read -r major minor patch <<<"$entry"
        if (( major > best_major || (major == best_major && minor > best_minor) )); then
            best_major=$major best_minor=$minor
        fi
    done
    (( best_major >= 0 )) && echo "$best_major $best_minor"
    return 0
}

check() {
    local version="$1" channel="$2" replace="$3"
    [[ "$version" =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)$ ]] \
        || die "the version $version is not <major>.<minor>.<patch> (such as 1.1.0 or 1.0.1)"
    local major=$((10#${BASH_REMATCH[1]})) minor=$((10#${BASH_REMATCH[2]})) patch=$((10#${BASH_REMATCH[3]}))
    read_published
    if [[ "$replace" == true ]]; then
        local kept=() entry
        for entry in "${published[@]+"${published[@]}"}"; do
            [[ "$entry" == "$major $minor $patch" ]] || kept+=("$entry")
        done
        published=("${kept[@]+"${kept[@]}"}")
    fi
    local stable newest_major newest_minor
    stable="$(newest_stable)"
    case "$channel" in
        stable)
            (( patch == 0 )) \
                || die "a stable release moves only major.minor and has patch 0, so $version cannot be stable; release $major.$((minor + 1)).0 as stable, or $version as a pre-release"
            if [[ -n "$stable" ]]; then
                read -r newest_major newest_minor <<<"$stable"
                if (( major < newest_major || (major == newest_major && minor <= newest_minor) )); then
                    die "the newest stable release is $newest_major.$newest_minor.0, so a stable release moves major.minor above it ($newest_major.$((newest_minor + 1)).0 or $((newest_major + 1)).0.0), not to $version"
                fi
            fi
            ;;
        prerelease)
            (( patch > 0 )) \
                || die "a pre-release moves only the patch, which is above 0, so $version cannot be a pre-release; release it as stable, or the next patch as a pre-release"
            if [[ -n "$stable" ]]; then
                read -r newest_major newest_minor <<<"$stable"
                if (( major != newest_major || minor != newest_minor )); then
                    die "the newest stable release is $newest_major.$newest_minor.0, so pre-releases are $newest_major.$newest_minor.<patch> until the next stable release; $version is not"
                fi
            fi
            local highest
            highest="$(highest_patch "$major" "$minor")"
            if (( patch <= highest )); then
                die "$major.$minor.$highest is already published, so the next pre-release on $major.$minor is $major.$minor.$((highest + 1)), not $version"
            fi
            ;;
        *) die "the channel is stable or prerelease, not $channel" ;;
    esac
    echo "$version follows the $channel version rules"
}

next_prerelease() {
    read_published
    local base major minor
    base="$(newest_stable)"
    [[ -n "$base" ]] || base="$(newest_any)"
    [[ -n "$base" ]] || die "no release is published yet, so there is no pre-release to follow"
    read -r major minor <<<"$base"
    local highest
    highest="$(highest_patch "$major" "$minor")"
    echo "$major.$minor.$((highest + 1))"
}

case "${1:-}" in
    check)
        [[ $# -eq 3 || ( $# -eq 4 && "$4" == --replace ) ]] || die "$usage"
        replace=false
        [[ $# -eq 4 ]] && replace=true
        check "$2" "$3" "$replace"
        ;;
    next)
        [[ $# -eq 2 && "$2" == prerelease ]] || die "$usage"
        next_prerelease
        ;;
    *) die "$usage" ;;
esac
