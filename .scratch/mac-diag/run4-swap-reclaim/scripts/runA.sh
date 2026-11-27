up() { cut -d' ' -f1 /proc/uptime; }
echo "START A uptime $(up) boot_completed=$(getprop sys.boot_completed)"
cat /proc/swaps
swapoff /dev/block/zram0; echo "swapoff rc=$? uptime $(up)"
while :; do
  echo "== uptime $(up) boot_completed=$(getprop sys.boot_completed)"
  grep -E 'MemFree|MemAvailable|SwapTotal|SwapFree' /proc/meminfo; cat /proc/swaps
  [ "$(cut -d. -f1 /proc/uptime)" -ge 930 ] && break
  sleep 60
done
echo "END A uptime $(up)"
