echo "START A2 uptime $(cut -d' ' -f1 /proc/uptime) boot_completed=$(getprop sys.boot_completed)"
cat /proc/swaps
swapoff /dev/block/zram0; echo "swapoff rc=$? uptime $(cut -d' ' -f1 /proc/uptime)"
i=0
while [ $i -lt 18 ]; do
  echo "== uptime $(cut -d' ' -f1 /proc/uptime) boot_completed=$(getprop sys.boot_completed)"
  grep -E 'MemTotal|MemFree|MemAvailable|SwapTotal|SwapFree' /proc/meminfo; cat /proc/swaps
  sleep 60; i=$((i+1))
done
echo "END A2 uptime $(cut -d' ' -f1 /proc/uptime)"
