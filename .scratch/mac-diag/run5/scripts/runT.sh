. /data/local/tmp/common.sh
echo "START T uptime $(up)"; blockreclaim
tick() { echo "64M swappiness=30" > /data/local/tmp/cg/memory.reclaim 2>/dev/null; echo "trim write rc=$? uptime $(up)"; }
logloop 30; echo "END T uptime $(up)"
