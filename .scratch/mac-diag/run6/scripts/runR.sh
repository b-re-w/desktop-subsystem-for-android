. /data/local/tmp/common.sh
echo "START R uptime $(up)"; S=/data/local/tmp/rereport.sink; : > $S
mount --bind $S /sys/module/page_reporting/parameters/rereport; echo "bind rereport rc=$? uptime $(up)"; grep rereport /proc/1/mountinfo
logloop 0; echo "END R uptime $(up)"
