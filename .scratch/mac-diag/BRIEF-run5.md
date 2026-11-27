# run5: narrow the Mac guest kernel panic (same rules as run3/BRIEF.md)

Owner's instruction (via the Windows session): keep narrowing the cause. run4 (3b2cca59):
swap off x4 and memory.reclaim blocked x4 had no panic; control 2 of 4 panicked.

1. Bigger samples: control (nothing changed) at least 6 more boots, so the control panic rate
   is known. Then repeat A and B as needed so the comparison means something.
2. Capture the full call stack of every panic and of the first warning before it (from
   android-console.log): which function, and whether it is in zram write/compress,
   swap-in, free page reporting, or the balloon. Keep the raw console per boot.
3. Splitting controls, run as many as you judge useful:
   - zram compression algorithm changed (swapoff /dev/block/zram0, reset, set
     comp_algorithm to another available one, mkswap, swapon) early after boot;
   - free page reporting off if it can be turned off at run time as root (check
     /sys/module/page_reporting/parameters); record how;
   - swap on but only the idle reclaim (swappiness=200) blocked vs only the trim blocked, if
     you can separate them.
4. Root: use `dsa apps root com.android.shell remember` (needs ONE owner approval on the Mac:
   send a PushNotification first, run it exactly once, never retry in a loop). Set it back to
   ask at the end.
5. Do not install anything else. Image stays 17.0.0-16373615-15.1-dev.10-arm64, host dev.11.
   Note: develop now has #1045 (9bc71d84, trim swappiness 0) and #1043 (drm shmem dirty);
   the installed image has neither.
6. Deliver in `.scratch/mac-diag/run5/` with RESULT.txt, commit to diag/mac-start, push, and
   tell the DSA MAC session (pane w1:p21) the commit and a 5-line summary.
