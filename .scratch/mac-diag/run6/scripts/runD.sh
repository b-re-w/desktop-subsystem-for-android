. /data/local/tmp/common.sh
echo "START D uptime $(up)"; S=/data/local/tmp/dropcaches.sink; : > $S
mount --bind $S /proc/sys/vm/drop_caches; echo "bind drop_caches rc=$? uptime $(up)"; grep drop_caches /proc/1/mountinfo
logloop 0; echo "END D uptime $(up)"
