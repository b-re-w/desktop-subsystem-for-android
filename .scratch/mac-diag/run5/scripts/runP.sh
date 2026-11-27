. /data/local/tmp/common.sh
echo "START P uptime $(up)"; B=/sys/bus/virtio/drivers/virtio_balloon
ls $B; cat /sys/module/page_reporting/parameters/reported_total_kb
for d in $(ls $B | grep '^virtio'); do echo $d > $B/unbind; echo "unbind $d rc=$? uptime $(up)"; done
ls $B
logloop 0; cat /sys/module/page_reporting/parameters/reported_total_kb; echo "END P uptime $(up)"
