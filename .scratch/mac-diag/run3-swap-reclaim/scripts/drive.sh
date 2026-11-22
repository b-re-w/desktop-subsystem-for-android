#!/bin/zsh
# usage: drive.sh <guest script name> <out dir> <start marker>
A=(adb -P 5038 -s 127.0.0.1:58526); out=$2; L="$HOME/Library/Application Support/DSA/logs"
for n in 1 2 3 4 5 6 7 8; do
  adb -P 5038 connect 127.0.0.1:58526 >/dev/null 2>&1
  echo "--- su attempt $n at $(date -u +%H:%M:%S)" >> $out/su.out
  $A shell su 0 sh /data/local/tmp/$1 >> $out/su.out 2>&1
  grep -q "$3" $out/su.out && break
  sleep 3
done
echo "--- su session ended at $(date -u +%H:%M:%S)" >> $out/su.out
cp "$L/android-console.log" $out/android-console.txt
[ -f "$L/android-console.1.log" ] && cp "$L/android-console.1.log" $out/android-console.1.txt
grep root: "$L/dsa-hostd.log" | tail -12 > $out/hostd-root.txt
