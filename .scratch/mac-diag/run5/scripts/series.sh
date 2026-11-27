#!/bin/zsh
# usage: series.sh <run> ... (C1 Z1 ...). One su call per boot, never retried.
cd ${0:h}/..
D=/Applications/DSA.app/Contents/MacOS/dsa; A=(adb -P 5038 -s 127.0.0.1:58526); L="$HOME/Library/Application Support/DSA/logs"
for run in "$@"; do
  t=${run[1]}; o=$run; mkdir -p $o
  $D stop --wait >/dev/null 2>&1
  $D start --wait > $o/start.txt 2>&1
  echo "started $(date -u +%FT%T)" >> $o/start.txt
  for i in 1 2 3 4 5 6 7 8 9 10; do adb -P 5038 connect 127.0.0.1:58526 >/dev/null 2>&1; $A shell true 2>/dev/null && break; sleep 1; done
  $A push scripts/common.sh /data/local/tmp/common.sh >/dev/null 2>&1
  $A push scripts/run$t.sh /data/local/tmp/run$t.sh >/dev/null 2>&1
  $A shell su 0 sh /data/local/tmp/run$t.sh > $o/su.out 2>&1 &
  p=$!; ( sleep 1500; kill $p 2>/dev/null ) & w=$!
  wait $p; kill $w 2>/dev/null
  echo "--- su session ended $(date -u +%FT%T)" >> $o/su.out
  sleep 5
  cp "$L/android-console.log" $o/android-console.txt
  grep root: "$L/dsa-hostd.log" | tail -1 > $o/hostd-root.txt
  panic=$(grep -m1 -oE "^\[ *[0-9.]+\] Kernel panic.*" $o/android-console.txt)
  first=$(grep -m1 -E "BUG:|kernel BUG|WARNING:|Unable to handle" $o/android-console.txt | cut -c1-140)
  echo "$run | started=$(grep -c "^START $t" $o/su.out) ended=$(grep -c "^END $t" $o/su.out) | last $(grep '^== uptime' $o/su.out | tail -1 | cut -d' ' -f2-3) | panic: ${panic:-none} | first sign: ${first:-none}" >> summary.txt
  if ! grep -q "^START $t" $o/su.out && [ -z "$panic" ]; then echo "$run: su did not start and no panic; stopping the series" >> summary.txt; break; fi
done
echo "series done $(date -u +%FT%T)" >> summary.txt
