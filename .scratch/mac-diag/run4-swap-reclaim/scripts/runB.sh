up() { cut -d' ' -f1 /proc/uptime; }
echo "START B uptime $(up) boot_completed=$(getprop sys.boot_completed) reclaim=$(getprop vendor.dsa.memory.reclaim) trim=$(getprop vendor.dsa.memory.trim)"
T=/sys/fs/cgroup/memory.reclaim
S=/data/local/tmp/reclaim.sink; : > $S; chmod 666 $S
mount --bind $S $T; echo "bind rc=$? uptime $(up)"; grep memory.reclaim /proc/1/mountinfo
r1=0; t1=0; n=0
while :; do
  if [ $((n % 120)) = 0 ]; then
    echo "== uptime $(up) boot_completed=$(getprop sys.boot_completed) reclaim_seen=$r1 trim_seen=$t1 sink_bytes=$(wc -c < $S)"
    grep -E 'MemFree|MemAvailable|SwapTotal|SwapFree' /proc/meminfo; cat /proc/swaps
    [ "$(cut -d. -f1 /proc/uptime)" -ge 930 ] && break
  fi
  [ "$(getprop vendor.dsa.memory.reclaim)" = 1 ] && r1=$((r1+1))
  [ "$(getprop vendor.dsa.memory.trim)" = 1 ] && t1=$((t1+1))
  sleep 0.5; n=$((n+1))
done
echo "sink content:"; cat $S
umount $T; echo "END B umount rc=$? uptime $(up)"
