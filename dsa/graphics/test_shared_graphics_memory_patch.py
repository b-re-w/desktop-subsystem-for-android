"""The graphics memory the host shares with the guest: host patch 0037, guest/mesa 0015, the
renderer backend and the hwcomposer.

A guest's host-visible memory goes into a region the service attaches to the VM, which the
renderer imports into Vulkan and the guest maps itself, so neither side copies it through the
stream; gralloc buffers' CPU uploads and read backs go through staging grants there. The
allocator and the guest's grants run as C++ tests on fakes (tests/graphics_memory_test.cpp);
these check that the patches and the backend use them where the copies were.
"""

import re
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
HOST_PATCH = next((HERE / "patches").glob("0037-*.patch"))
MESA_PATCH = next((HERE / "patches" / "guest" / "mesa").glob("0015-*.patch"))
HEADER = HERE / "include" / "dsa_graphics_memory.h"
DEVICE = HERE / "include" / "dsa_graphics_memory_device.h"
BACKEND = HERE / "host" / "dsa_gfx_backend.cpp"
SHARED = HERE / "host" / "dsa_gfx_shared_memory.cpp"
MEMORY = HERE / "host" / "dsa_gfx_memory.cpp"
HWC = HERE.parent / "guest" / "hwcomposer" / "src"


def read(path):
    return path.read_text(encoding="utf-8").replace("\r\n", "\n")


def added(patch, path):
    """The lines a patch adds to one file, without their '+'."""
    text = read(patch)
    start = text.index(f"+++ b/{path}\n")
    end = text.find("\ndiff --git ", start)
    body = text[start:end if end >= 0 else len(text)]
    return "\n".join(line[1:] for line in body.splitlines()[1:] if line.startswith("+"))


class HostPatchTests(unittest.TestCase):
    def test_pr7_the_host_scripts_apply_the_patch(self):
        for script in (HERE / "host" / "fetch.sh", HERE / "host" / "fetch.ps1"):
            self.assertIn("0037", read(script), script)

    def test_pr7_host_visible_memory_is_imported_from_the_shared_region(self):
        code = added(HOST_PATCH, "host/vulkan/VkDecoderGlobalState.cpp")
        self.assertIn("dsa::graphicsMemory().allocate(allocInfo.allocationSize, owner, &why)", code)
        self.assertIn("VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT", code)
        # The driver is asked whether it takes the pointer for the memory type, per allocation,
        # and a refusal gives the grant back and takes the copy path.
        self.assertIn("vk->vkGetMemoryHostPointerPropertiesEXT(", code)
        self.assertIn("!(properties.memoryTypeBits & (1u << allocInfo.memoryTypeIndex))", code)
        refusal = code[code.index("!(properties.memoryTypeBits"):]
        self.assertLess(refusal.index("dsa::graphicsMemory().release(grant->offset);"),
                        refusal.index("return std::nullopt;"))
        # Only for a device whose guest maps the region, and never an image's own memory.
        self.assertIn("if (!deviceInfo || !deviceInfo->dsaMapsSharedMemory) return std::nullopt;", code)
        self.assertIn("VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO", code)
        # Each reason is said once.
        self.assertIn("if (said.fetch_or(bit) & bit) return;", code)
        # Buffers say they may be bound to imported host memory.
        self.assertIn("(dsaDeviceMapsSharedMemory(device) &&", code)

    def test_pr7_a_freed_allocation_gives_its_grant_back(self):
        patch = read(HOST_PATCH)
        free = patch[patch.index(" deviceDispatch->vkFreeMemory(device, memory, pAllocator);\n+"):]
        self.assertIn("dsa::graphicsMemory().release(memoryInfo.dsaSharedOffset,", free[:400])
        # Only a grant of the region as it is now: one from before a take over is not its to
        # revoke.
        self.assertIn("memoryInfo.dsaSharedGeneration);", free[:500])

    def test_pr7_the_grant_goes_only_to_the_memorys_own_device(self):
        code = added(HOST_PATCH, "host/vulkan/VkDecoderGlobalState.cpp")
        self.assertIn("*pAddress = dsa::kGraphicsMemoryHello;", code)
        self.assertIn("deviceInfo->dsaGuestProcess = dsa::currentGraphicsOwner();", code)
        self.assertIn("if (!info || !info->dsaShared || info->device != device) {", code)
        self.assertIn("*pHostmemId = info->dsaSharedKey;", code)

    def test_pr7_no_bytes_cross_the_stream_for_shared_memory(self):
        code = added(HOST_PATCH, "host/vulkan/VkDecoder.cpp")
        # A flush of shared memory copies nothing; an invalidate sends size 0.
        self.assertEqual(code.count("if (dsa::graphicsMemory().offsetOf(hostPtr)) {"), 2)
        self.assertIn("*readStreamPtrPtr += readStream;\n                            continue;", code)
        self.assertIn("vkStream->write(&writeStream, sizeof(uint64_t));\n                            continue;", code)

    def test_pr7_a_staging_copy_touches_only_the_callers_own_grant(self):
        code = added(HOST_PATCH, "host/RenderControl.cpp")
        self.assertIn("dsa::graphicsMemory().owns(pixels, needed, info->m_puid)", code)
        self.assertIn("!dsaOwnsPixels(pixels, pixels_size, width, height, format, type)", code)
        self.assertIn("glStr += dsa::kGraphicsMemoryExtension;", code)
        stream = added(HOST_PATCH, "host/ChannelStream.cpp")
        self.assertIn("(guest_paddr & dsa::kGraphicsDmaTagMask) == dsa::kGraphicsDmaTag", stream)


class BackendTests(unittest.TestCase):
    def test_pr7_the_embedder_hands_the_renderer_the_region(self):
        self.assertIn("dsa_gfx_set_shared_graphics_memory", read(HERE / "host" / "dsa_gfx_backend.h"))
        shared = read(SHARED)
        self.assertIn("memory.attach(data, data_length, grants, grants_length, scrub, nullptr, commit, nullptr,", shared)
        # Freed pages are zeroed for the next app, then given back.
        scrub = shared[shared.index("void scrub("):]
        self.assertLess(scrub.index("std::memset(at, 0, length);"), scrub.index("DiscardVirtualMemory("))
        self.assertIn("dsa_gfx_shared_memory.cpp", read(HERE / "host" / "CMakeLists.txt"))

    def test_pr7_the_sections_only_reserve_and_a_grant_is_committed_before_its_key(self):
        # The service makes both graphics sections reserved, never committed whole.
        region = read(HERE.parent / "host" / "platform" / "windows" / "src" / "graphics_region.rs")
        self.assertIn("PAGE_READWRITE | SEC_RESERVE", region)
        self.assertNotIn("SEC_COMMIT", region)
        # The renderer commits in its own view, and a refusal is said once.
        shared = read(SHARED)
        commit = shared[shared.index("bool commit("):]
        self.assertIn("VirtualAlloc(at, static_cast<SIZE_T>(length), MEM_COMMIT, PAGE_READWRITE)", commit)
        self.assertIn("s_commit_failure_said.exchange(true)", commit)
        # The allocator commits the granules before the key that makes the grant visible.
        header = read(HEADER)
        allocate = header[header.index("std::optional<GraphicsGrant> allocateLocked("):]
        self.assertLess(allocate.index("commitLocked(start, count)"),
                        allocate.index("sharedStore(at + kGrantKeyAt, key);"))
        self.assertIn("GrantRefusal::CommitFailed", allocate)
        # Its own one-time log line in the Vulkan path, apart from the other reasons.
        self.assertIn("dsaSayCopied(10 + static_cast<int>(why), dsa::grantRefusalText(why));",
                      added(HOST_PATCH, "host/vulkan/VkDecoderGlobalState.cpp"))

    def test_pr7_an_ended_process_and_the_trim_take_grants_back(self):
        backend = read(BACKEND)
        self.assertIn("dsa::graphicsMemory().releaseOwner(puid);", backend)
        self.assertIn("dsa::graphicsOwnerLookup().store(render_thread_process);", backend)
        self.assertIn("if (op != dsa::kStagingRequest) {", backend)
        self.assertIn("dsa::graphicsMemory().reclaim();", read(MEMORY))


class GuestPatchTests(unittest.TestCase):
    def test_pr7_shared_memory_has_no_copy_hash_flush_or_read_back(self):
        code = added(MESA_PATCH, "src/gfxstream/guest/vulkan_enc/ResourceTracker.cpp")
        # The shared branch returns before a copy is made, so the memory is never shadowed:
        # the flush, the read back and the hashes all look only at shadowed memory.
        self.assertIn("if (uint8_t* shared = mapSharedMemory(enc, device, mem, &sharedOffset, &sharedLength)) {", code)
        branch = code[code.index("if (uint8_t* shared = mapSharedMemory("):]
        self.assertIn("stored.sharedRegion = true;", branch[:600])
        self.assertIn("return host_res;", branch[:700])
        self.assertNotIn("shadowed = true", branch[:700])
        tracker = read(HERE / "patches" / "0003-copy-host-visible-memory-without-shared-memory.patch")
        self.assertIn("if (!info.shadowed || info.device != device || !info.shadowBlocks) continue;", tracker)
        # Nor does the kernel track its pages: only a copy is tracked.
        self.assertIn("bool sharedRegion = false;",
                      added(MESA_PATCH, "src/gfxstream/guest/vulkan_enc/ResourceTracker.h"))
        encoder = added(MESA_PATCH, "src/gfxstream/guest/vulkan_enc/VkEncoder.cpp")
        self.assertIn("if (!hostPtr || sResourceTracker->isSharedMemory(memory)) {", encoder)

    def test_pr7_the_guest_asks_once_and_checks_the_region(self):
        code = added(MESA_PATCH, "src/gfxstream/guest/vulkan_enc/ResourceTracker.cpp")
        self.assertIn("if (!usingDirectMapping()) askSharedMemory(enc, device);", code)
        self.assertIn("if (it->second.sharedMemoryAsked) return it->second.mapsSharedMemory;", code)
        self.assertIn("regionId == shared->info().regionId && length == shared->info().length;", code)

    def test_pr7_the_guest_lets_go_before_the_host_hears_of_the_free(self):
        code = added(MESA_PATCH, "src/gfxstream/guest/vulkan_enc/ResourceTracker.cpp")
        self.assertIn("} else if (info.sharedRegion) {", code)
        self.assertIn("shared->unmap(info.ptr, info.sharedOffset, info.sharedLength);", code)
        device = read(DEVICE)
        unmap = device[device.index("void unmap("):]
        self.assertLess(unmap.index("munmap(at, length);"), unmap.index("kGraphicsMemoryReleaseIoctl"))

    def test_pr7_gralloc_uploads_and_read_backs_go_through_staging(self):
        gralloc = added(MESA_PATCH, "src/gfxstream/guest/android/GrallocMinigbm.cpp")
        self.assertIn("rcEnc->rcReadColorBufferDMA(", gralloc)
        self.assertIn("rcEnc->rcUpdateColorBufferDMA(", gralloc)
        self.assertIn("rcEnc->bindDmaDirectly(staging->ptr, staging->dmaAddress());", gralloc)
        # Without the host's extension, or a staging grant large enough, the stream as before.
        self.assertIn("hostSharesGraphicsMemory(rcEnc)", gralloc)
        hwc = read(HWC / "host_renderer.cpp")
        self.assertIn("rc->rcUpdateColorBufferDMA(", hwc)
        self.assertIn("const dsa::StagingRequest request{dsa::kStagingRequest, 0, size};", hwc)
        self.assertIn("packed = host.stagingFor(layout.packedSize());", read(HWC / "cpu_upload.cpp"))

    def test_pr7_the_patches_are_lf_and_end_with_a_newline(self):
        for patch in (HOST_PATCH, MESA_PATCH):
            data = patch.read_bytes()
            self.assertTrue(data.endswith(b"\n"), patch)
            self.assertTrue(re.search(rb"^diff --git ", data, re.MULTILINE), patch)


if __name__ == "__main__":
    unittest.main()
