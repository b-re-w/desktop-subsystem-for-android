echo "START B uptime $(cut -d' ' -f1 /proc/uptime) boot_completed=$(getprop sys.boot_completed) reclaim=$(getprop vendor.dsa.memory.reclaim) trim=$(getprop vendor.dsa.memory.trim)"
T=/sys/fs/cgroup/memory.reclaim
S=/data/local/tmp/reclaim.sink; : > $S; chmod 666 $S
mount --bind $S $T; echo "bind rc=$? uptime $(cut -d' ' -f1 /proc/uptime)"; grep memory.reclaim /proc/1/mountinfo
setprop vendor.dsa.memory.reclaim 0
i=0; r1=0; t1=0
while [ $i -lt 18 ]; do
  echo "== uptime $(cut -d' ' -f1 /proc/uptime) boot_completed=$(getprop sys.boot_completed) reclaim_seen=$r1 trim_seen=$t1 sink_bytes=$(wc -c < $S)"
  grep -E 'MemTotal|MemFree|MemAvailable|SwapTotal|SwapFree' /proc/meminfo; cat /proc/swaps
  j=0
  while [ $j -lt 120 ]; do
    if [ "$(getprop vendor.dsa.memory.reclaim)" = 1 ]; then r1=$((r1+1)); setprop vendor.dsa.memory.reclaim 0; echo "reclaim=1 seen, set 0, uptime $(cut -d' ' -f1 /proc/uptime)"; fi
    if [ "$(getprop vendor.dsa.memory.trim)" = 1 ]; then t1=$((t1+1)); echo "trim=1 seen uptime $(cut -d' ' -f1 /proc/uptime)"; fi
    sleep 0.5; j=$((j+1))
  done
  i=$((i+1))
done
echo "sink content:"; cat $S
umount $T; echo "END B umount rc=$? uptime $(cut -d' ' -f1 /proc/uptime)"
