#!/bin/zsh
# Boots the KASAN kernel repeatedly, 15 minutes each, and keeps every console. Stops after
# the second boot that has a KASAN report. No root is used.
cd ${0:h}/../kasan
D=/Applications/DSA.app/Contents/MacOS/dsa; A=(adb -P 5038 -s 127.0.0.1:58526); L="$HOME/Library/Application Support/DSA/logs"
hits=0
for n in $(seq 1 16); do
  o=boot$n; mkdir -p $o
  $D stop --wait >/dev/null 2>&1
  $D start --wait --timeout 900 > $o/start.txt 2>&1
  echo "started $(date -u +%FT%T)" >> $o/start.txt
  adb -P 5038 connect 127.0.0.1:58526 >/dev/null 2>&1
  t0=$(date +%s)
  while [ $(( $(date +%s) - t0 )) -lt 930 ]; do
    grep -qE "Kernel panic" "$L/android-console.log" && break
    $D status --no-start 2>/dev/null | head -1 | grep -q "Stopped" && break
    echo "$(date -u +%T) up=$($A shell cut -d' ' -f1 /proc/uptime 2>/dev/null) $($A shell 'grep -E "MemFree|SwapFree" /proc/meminfo' 2>/dev/null | tr -s ' \n' ' ')" >> $o/meminfo.txt
    sleep 60
  done
  sleep 3
  cp "$L/android-console.log" $o/android-console.txt
  k=$(grep -c "BUG: KASAN" $o/android-console.txt)
  panic=$(grep -m1 -oE "^\[ *[0-9.]+\] Kernel panic.*" $o/android-console.txt | cut -c1-90)
  first=$(grep -E "BUG:|kernel BUG|WARNING:|Unable to handle" $o/android-console.txt | grep -v "slab_common.c:109" | head -1 | cut -c1-150)
  echo "boot$n | kasan reports=$k | panic: ${panic:-none} | first sign: ${first:-none}" >> ../kasan-summary.txt
  [ $k -gt 0 ] && hits=$((hits+1))
  [ $hits -ge 2 ] && break
done
echo "kasan series done $(date -u +%FT%T)" >> ../kasan-summary.txt
