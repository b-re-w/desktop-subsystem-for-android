up() { cut -d' ' -f1 /proc/uptime; }
logloop() {  # $1: seconds between calls of "tick" (0 = none)
  n=0
  while :; do
    echo "== uptime $(up)"
    grep -E 'MemFree|MemAvailable|SwapTotal|SwapFree' /proc/meminfo; cat /proc/swaps
    [ "$(cut -d. -f1 /proc/uptime)" -ge 930 ] && break
    if [ "$1" = 30 ]; then tick; sleep 30; tick; sleep 30; elif [ "$1" = 60 ]; then tick; sleep 60; else sleep 60; fi
  done
}
blockreclaim() {
  S=/data/local/tmp/reclaim.sink; : > $S
  mkdir -p /data/local/tmp/cg; mount -t cgroup2 none /data/local/tmp/cg; echo "cg mount rc=$?"
  mount --bind $S /sys/fs/cgroup/memory.reclaim; echo "bind rc=$? uptime $(up)"
}
