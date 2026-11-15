# Changelog

## v1.0.0 (2026-11-16)

The first stable release of Desktop Subsystem for Android™ (DSA) on Windows 11, x64 and ARM64.

### Android in its own VM
- Android 16 and 17 run in a dedicated lightweight VM created through the Host Compute
  Service, with DSA's own kernel (binder built in). The user's WSL is never touched.
- Idle memory goes back to Windows: Android dozes when no window is open, free pages are
  reported to the host, and closed apps' memory is reclaimed.
- Signed Android image and DSA updates; DSA updates have a stable and a pre-release channel.

### One window per Android task
- Every Android task is a native window with its own taskbar entry, icon and title bar,
  snap layouts, Alt+Tab and per-monitor density.
- App overlays, toasts and picture-in-picture appear as their own windows.
- Host text input through the Windows input method (TSF), with Korean composition inline.

### Graphics
- OpenGL ES through ANGLE and Vulkan remoted to the host GPU (gfxstream), with shared memory
  for host-visible buffers and video, GPU reset recovery, and a Dozen fallback for GPUs
  without native Vulkan.

### Desktop integration
- Start menu entries, notifications in the Windows notification center, drag and drop of
  files, an APK installer, desktop widgets, networking with the host's DNS.
- Optional ARM translation and an optional Google Play installer; DSA ships neither Google
  software nor a store.

### Settings and command line
- The DSA settings app in 14 languages, and the `dsa` command for everything it does.

### Not yet released
- Linux and macOS hosts are in development.

Android is a trademark of Google LLC.
