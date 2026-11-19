#!/usr/bin/env bash
# Checks the macOS installer's Distribution file: it shows the artwork, a welcome page, the
# licence and a conclusion page, and every file it names exists, in every language the
# Windows installer has a .wxl for.
set -euo pipefail

root="$(cd "$(dirname "$0")/../.." && pwd)"
dir="$root/dsa/host/platform/macos/packaging/installer"
dist="$dir/Distribution.xml"
res="$dir/resources"
fail() { echo "macOS installer design: $*" >&2; exit 1; }

for element in background background-darkAqua welcome license conclusion; do
  grep -Eq "<$element file=\"[^\"]+\"" "$dist" || fail "Distribution.xml has no <$element>"
done
file_of() { sed -nE "s/.*<$1 file=\"([^\"]+)\".*/\1/p" "$dist"; }

for element in background background-darkAqua; do
  test -f "$res/$(file_of "$element")" || fail "$(file_of "$element") is missing from resources"
done
# build-app.sh copies the repository's LICENSE in as the licence page.
[[ "$(file_of license)" == LICENSE.txt ]] || fail "the licence page is not LICENSE.txt"
grep -Fq 'cp "$root/LICENSE" "$work/resources/LICENSE.txt"' "$dir/../build-app.sh" \
  || fail "build-app.sh does not copy LICENSE into the resources"
test -f "$root/LICENSE" || fail "LICENSE is missing"

count=0
for wxl in "$root"/dsa/host/platform/windows/installer/dsa.*.wxl; do
  culture="$(basename "$wxl" .wxl)"; culture="${culture#dsa.}"
  case "$culture" in
    en-us) lang=en ;;
    pt-br) lang=pt_BR ;;
    zh-cn) lang=zh_CN ;;
    zh-tw) lang=zh_TW ;;
    *) lang="${culture%%-*}" ;;
  esac
  for element in welcome conclusion; do
    page="$res/$lang.lproj/$(file_of "$element")"
    test -f "$page" || fail "$lang.lproj has no $(file_of "$element")"
    grep -Fq 'Google LLC' "$page" || fail "$page does not carry the trademark notice"
  done
  grep -Fq 'Android™' "$res/$lang.lproj/welcome.html" || fail "$lang welcome does not name Android with ™"
  count=$((count + 1))
done
[[ $count -gt 0 ]] || fail "no Windows .wxl files found"
echo "macOS installer design: ok ($count languages)"
