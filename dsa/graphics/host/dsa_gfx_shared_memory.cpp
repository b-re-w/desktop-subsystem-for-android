/*
 * Copyright 2026 The DSA Authors
 * SPDX-License-Identifier: Apache-2.0
 *
 * The graphics memory the host shares with the guest (dsa_graphics_memory.h): handing the
 * embedder's mapping of the two sections to the renderer's allocator, and giving the pages of
 * a range nothing maps any more back to the system.
 *
 * Kept apart from dsa_gfx_backend.cpp so the platform headers it needs meet no gfxstream
 * header.
 */

#include "dsa_gfx_backend.h"

#include <atomic>
#include <cstdio>
#include <cstring>

#include "dsa_graphics_memory.h"

#ifdef _WIN32
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#ifndef NOMINMAX
#define NOMINMAX
#endif
#include <windows.h>
#else
#include <sys/mman.h>
#endif

void dsa_gfx_log_line(char severity, const char* message);

namespace {

std::atomic<bool> s_scrub_failure_said{false};

/* Zeroes the range, so the next app it is granted to never reads what the last one wrote, and
 * gives its pages back: Windows may then take them, and a page it took reads as zeros. */
void scrub(void*, uint8_t* at, uint64_t length) {
    std::memset(at, 0, length);
#ifdef _WIN32
    const DWORD code = DiscardVirtualMemory(at, static_cast<SIZE_T>(length));
    if (code != ERROR_SUCCESS && !s_scrub_failure_said.exchange(true)) {
        char line[160];
        std::snprintf(line, sizeof(line),
                      "graphics memory: freed shared pages were zeroed but not given back to "
                      "Windows (error %lu); they go with the VM",
                      static_cast<unsigned long>(code));
        dsa_gfx_log_line('W', line);
    }
#else
    /* A shared mapping of a memfd: punching the range out frees its pages, which then read as
     * zeros. */
    if (madvise(at, length, MADV_REMOVE) != 0 && !s_scrub_failure_said.exchange(true)) {
        dsa_gfx_log_line('W',
                         "graphics memory: freed shared pages were zeroed but not given back; "
                         "they go with the VM");
    }
#endif
}

}  // namespace

extern "C" DSA_GFX_EXPORT int dsa_gfx_set_shared_graphics_memory(uint8_t* data,
                                                                 uint64_t data_length,
                                                                 uint8_t* grants,
                                                                 uint64_t grants_length) {
    dsa::GraphicsMemory& memory = dsa::graphicsMemory();
    if (!data || !grants || data_length == 0) {
        memory.detach();
        return 0;
    }
    if (!memory.attach(data, data_length, grants, grants_length, scrub, nullptr)) {
        dsa_gfx_log_line('W',
                         "graphics memory: the memory the service shares is not a grants table "
                         "this renderer knows; host-visible memory is copied through the stream");
        return -1;
    }
    /* The embedder says in its own log that the region is in use. */
    return 0;
}

extern "C" DSA_GFX_EXPORT int dsa_gfx_get_shared_graphics_memory_stats(
    dsa_gfx_shared_memory_stats* stats) {
    if (!stats) return -1;
    dsa::GraphicsMemory& memory = dsa::graphicsMemory();
    memory.reclaim();
    const dsa::GraphicsMemory::Stats now = memory.stats();
    stats->region_bytes = now.regionBytes;
    stats->granted_bytes = now.grantedBytes;
    stats->held_bytes = now.heldBytes;
    stats->grants = now.grants;
    stats->reserved = 0;
    return now.regionBytes ? 0 : -1;
}
