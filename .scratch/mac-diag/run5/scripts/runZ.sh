. /data/local/tmp/common.sh
echo "START Z uptime $(up)"; Z=/sys/block/zram0
cat /proc/swaps; cat $Z/comp_algorithm
swapoff /dev/block/zram0; echo "swapoff rc=$?"
echo 1 > $Z/reset; echo "reset rc=$?"
echo lzo-rle > $Z/comp_algorithm; echo "alg rc=$?"
echo 2072051712 > $Z/disksize; echo "disksize rc=$?"
mkswap /dev/block/zram0; echo "mkswap rc=$?"; swapon /dev/block/zram0; echo "swapon rc=$? uptime $(up)"
cat $Z/comp_algorithm; cat $Z/recomp_algorithm 2>&1
logloop 0; cat $Z/comp_algorithm; cat $Z/mm_stat; echo "END Z uptime $(up)"
