. /data/local/tmp/common.sh
echo "START K uptime $(up)"; S=/data/local/tmp/zram.sink; : > $S
for f in idle recompress compact; do mount --bind $S /sys/devices/virtual/block/zram0/$f; echo "bind $f rc=$?"; done
echo "binds done uptime $(up)"
logloop 0; echo "END K uptime $(up)"
