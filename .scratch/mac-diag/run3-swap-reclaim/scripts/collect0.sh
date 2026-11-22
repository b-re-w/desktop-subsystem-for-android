cat /data/local/tmp/runA.log; echo "== now uptime $(cut -d' ' -f1 /proc/uptime)"; grep -E 'MemFree|SwapTotal|SwapFree' /proc/meminfo; cat /proc/swaps
