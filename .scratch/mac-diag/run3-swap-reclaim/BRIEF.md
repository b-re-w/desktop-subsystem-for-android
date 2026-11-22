# Task: Mac guest kernel panic, run3 (swap / reclaim)

Reply to the owner in Korean. No em dashes. No AI attribution in commits. Do not change code,
workflows or branches other than `diag/mac-start`. Do not open PRs. Never touch non-DSA windows.
adb only as `adb -P 5038 -s 127.0.0.1:58526`; never send anything to device R5CT83X3RGF.
Before anything that needs the owner (the guest su approval dialog on the Mac), send a
PushNotification first.

## Background
- Host DSA dev.11 (/Applications/DSA.app, CLI `/Applications/DSA.app/Contents/MacOS/dsa`),
  image 17.0.0-16373615-15.1-dev.10-arm64, VM 2048 MB fixed, 2 vCPU.
- The guest kernel panics within minutes with memory corruption (rb-tree, anon_vma, slab).
  Control run: panic at 455 s (warning at 436 s in swap_cache_add_folio from
  user_proactive_reclaim, Oops in __kmem_cache_alloc_bulk). MemFree was ~1.5 GB: not OOM.
- Guest console: `~/Library/Application Support/DSA/logs/android-console.log`; host log
  `dsa-hostd.log` beside it (lines starting `root:` show su approvals).
- Root: the image is a production build (no adb root). `su 0 sh <script>` works after the owner
  approves a dialog on the Mac; approvals have a short grace period. The root shell runs as
  u:r:dsa_su_root and can write /data/local/tmp. A process started with nohup/& dies when the
  su session ends, so keep the su session in the foreground for the whole test, and quote
  nothing on the su command line (pass a script path only).

## Tests (each 15 min after boot_completed, same conditions)
- A (swap off): ALREADY RUNNING, started by the previous session ~14:31 UTC. It runs
  /data/local/tmp/runA.sh (swapoff -a, then every 60 s meminfo + /proc/swaps into
  /data/local/tmp/runA.log). A watcher writes into
  `.scratch/mac-diag/run3-swap-reclaim/A-swapoff/` (su.out, meminfo-swaps.txt,
  android-console.txt) when it ends. Check those files; if missing, collect them yourself.
- B (block only the post-boot reclaim, swap on): read `dsa/guest/image/dsa-channeld.rc`
  lines 58-74 (trigger `vendor.dsa.memory.reclaim=1 && sys.boot_completed=1` writes
  `/sys/fs/cgroup/memory.reclaim ... swappiness=200` four times). Restart Android
  (`dsa stop`, `dsa start --wait`), and as root keep `vendor.dsa.memory.reclaim` at 0
  (a root loop that does `setprop vendor.dsa.memory.reclaim 0` whenever it reads 1, started as
  early as possible). Note the trim trigger at line 112 (`vendor.dsa.memory.trim`) also writes
  memory.reclaim; record whether it fired. Log meminfo and /proc/swaps every 60 s. Record the
  method used.
- Per test: panic yes/no and time (uptime), console log, meminfo/swaps log.

## Deliver
Put RESULT.txt plus raw files in `.scratch/mac-diag/run3-swap-reclaim/` in worktree
`.claude/worktrees/diag-mac` (branch `diag/mac-start`), `git add -f`, commit, `git push origin
diag/mac-start`. Then tell the Windows session (SendMessage to
`bridge:session_011jHset2zceFaGrzknmSWXR`) the commit, and give the owner a short Korean summary.
Afterwards leave Android stopped or running as you found it, and delete large temporary copies.
