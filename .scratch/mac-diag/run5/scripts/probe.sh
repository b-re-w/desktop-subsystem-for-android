cut -d' ' -f1 /proc/uptime
echo "--- zram"; cat /sys/block/zram0/comp_algorithm; cat /sys/block/zram0/recomp_algorithm 2>&1; cat /sys/block/zram0/disksize; cat /proc/swaps
echo "--- page_reporting"; for f in /sys/module/page_reporting/parameters/*; do echo "$f $(ls -l $f | cut -c1-10) = $(cat $f 2>&1)"; done
ls /sys/kernel/mm/ 2>&1; ls /sys/bus/virtio/drivers 2>&1; ls /sys/bus/virtio/drivers/virtio_balloon 2>&1
grep -iE "balloon|free.page|page_report" /proc/vmstat | head; cat /proc/cmdline
echo "--- second cgroup2 mount"; mkdir -p /data/local/tmp/cg; mount -t cgroup2 none /data/local/tmp/cg; echo rc=$?; ls -l /data/local/tmp/cg/memory.reclaim; echo "1M swappiness=0" > /data/local/tmp/cg/memory.reclaim; echo write rc=$?; umount /data/local/tmp/cg
echo "--- props"; getprop | grep -E "dsa.memory"
echo "--- avc from last B"; dmesg 2>/dev/null | grep -m2 avc
