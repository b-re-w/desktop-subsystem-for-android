// Copyright 2026 The DSA Authors
// SPDX-License-Identifier: Apache-2.0
//
// The guest Vulkan driver's line for a large grant of the shared graphics memory
// (dsa_driver_memory.h), run on the build machine.

#include "dsa_driver_memory.h"

#include <cstdio>
#include <cstring>

static int failures = 0;

#define EXPECT(cond)                                                                \
    do {                                                                            \
        if (!(cond)) {                                                              \
            std::fprintf(stderr, "%s:%d: expected %s\n", __FILE__, __LINE__, #cond); \
            ++failures;                                                             \
        }                                                                           \
    } while (0)

// nfr1: only a grant over 16 MiB gets a line of its own.
static void nfr1_only_a_grant_over_16_mib_is_logged_on_its_own() {
    EXPECT(!dsa::isLargeGrant(uint64_t(16) << 20));
    EXPECT(dsa::isLargeGrant((uint64_t(16) << 20) + 1));
    EXPECT(!dsa::isLargeGrant(1114112));
    EXPECT(dsa::isLargeGrant(uint64_t(64) << 20));
}

// nfr1: the line names the process, the size, the memory type and what it is for, so a large
// grant in the renderer's report can be traced to its caller.
static void nfr1_the_line_names_process_size_type_and_purpose() {
    char line[400];
    dsa::largeGrantLine(line, sizeof(line), true, "/system/bin/surfaceflinger", uint64_t(64) << 20,
                        2, "host-visible coherent cached",
                        "a CPU read back of a 4096x4096 gralloc buffer (staging)");
    EXPECT(std::strcmp(line,
                       "dsa-vk large grant: /system/bin/surfaceflinger made 64.0 MiB (67108864 "
                       "bytes) of the shared graphics memory, memory type 2 (host-visible "
                       "coherent cached), for a CPU read back of a 4096x4096 gralloc buffer "
                       "(staging)") == 0);
    dsa::largeGrantLine(line, sizeof(line), false, "com.example", uint64_t(20) << 20, 1,
                        "host-visible coherent", "an app's host-visible allocation");
    EXPECT(std::strstr(line, "com.example let go of 20.0 MiB (20971520 bytes)") != nullptr);
}

// nfr1: the process is the first word of its command line, which an app sets to its package.
static void nfr1_the_process_is_the_first_word_of_its_command_line() {
    char name[32];
    const char app[] = "com.android.settings\0\0\0";
    EXPECT(std::strcmp(dsa::processNameFromCmdline(app, sizeof(app), name, sizeof(name)),
                       "com.android.settings") == 0);
    const char flinger[] = "/system/bin/surfaceflinger\0";
    EXPECT(std::strcmp(dsa::processNameFromCmdline(flinger, sizeof(flinger), name, sizeof(name)),
                       "/system/bin/surfaceflinger") == 0);
    EXPECT(std::strcmp(dsa::processNameFromCmdline("", 0, name, sizeof(name)), "?") == 0);
    char small[5];
    EXPECT(std::strcmp(dsa::processNameFromCmdline(app, sizeof(app), small, sizeof(small)),
                       "com.") == 0);
    EXPECT(dsa::thisProcessName()[0] != '\0');
}

int main() {
    nfr1_only_a_grant_over_16_mib_is_logged_on_its_own();
    nfr1_the_line_names_process_size_type_and_purpose();
    nfr1_the_process_is_the_first_word_of_its_command_line();
    if (failures != 0) {
        std::fprintf(stderr, "%d failure(s)\n", failures);
        return 1;
    }
    std::printf("driver memory: all tests passed\n");
    return 0;
}
