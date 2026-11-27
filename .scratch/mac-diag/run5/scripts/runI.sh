. /data/local/tmp/common.sh
echo "START I uptime $(up)"; blockreclaim
tick() { r=""; for k in 1 2 3 4; do echo "64M swappiness=200" > /data/local/tmp/cg/memory.reclaim 2>/dev/null; r="$r$?"; done; echo "idle writes rc=$r uptime $(up)"; }
logloop 60; echo "END I uptime $(up)"
