// Copyright 2026 The DSA Authors
// SPDX-License-Identifier: Apache-2.0
//
// Why each allocation on a guest's device is what it is (dsa_allocation_note.h), run on the
// build machine.

#include "dsa_allocation_note.h"

#include <cstdio>
#include <string>
#include <thread>

static int failures = 0;

#define EXPECT(cond)                                                                \
    do {                                                                            \
        if (!(cond)) {                                                              \
            std::fprintf(stderr, "%s:%d: expected %s\n", __FILE__, __LINE__, #cond); \
            ++failures;                                                             \
        }                                                                           \
    } while (0)

static bool contains(const std::string& text, const char* part) {
    return text.find(part) != std::string::npos;
}

// nfr1: an allocation with no note was the renderer's own on the guest's device (a window
// image's staging buffer, a decoder's buffer), not the guest's vkAllocateMemory.
static void nfr1_an_allocation_without_a_note_is_the_renderers_own() {
    EXPECT(dsa::takeAllocationNote() == dsa::AllocationWhy::RendererOwn);
    EXPECT(contains(dsa::allocationWhyText(dsa::AllocationWhy::RendererOwn), "renderer"));
}

// nfr1: a note is taken by the next allocation on the same thread and by no later one.
static void nfr1_a_note_goes_to_the_next_allocation_only() {
    dsa::noteAllocation(dsa::AllocationWhy::DeviceNotShared);
    EXPECT(dsa::takeAllocationNote() == dsa::AllocationWhy::DeviceNotShared);
    EXPECT(dsa::takeAllocationNote() == dsa::AllocationWhy::RendererOwn);
}

// nfr1: a note made on one render thread never reaches another thread's allocation.
static void nfr1_a_note_stays_on_its_thread() {
    dsa::noteAllocation(dsa::AllocationWhy::Granted);
    dsa::AllocationWhy seen = dsa::AllocationWhy::Granted;
    std::thread other([&seen] { seen = dsa::takeAllocationNote(); });
    other.join();
    EXPECT(seen == dsa::AllocationWhy::RendererOwn);
    EXPECT(dsa::takeAllocationNote() == dsa::AllocationWhy::Granted);
}

// nfr1: both reasons that used to be silent are said: the device that never said it maps the
// shared memory, and memory a gfxstream feature allocates itself.
static void nfr1_the_silent_reasons_are_said() {
    EXPECT(contains(dsa::allocationWhyText(dsa::AllocationWhy::DeviceNotShared),
                    "never said this device maps"));
    EXPECT(contains(dsa::allocationWhyText(dsa::AllocationWhy::FeatureAllocates),
                    "gfxstream feature"));
    for (int why = 0; why <= static_cast<int>(dsa::AllocationWhy::ImportRefused); why++) {
        const std::string text = dsa::allocationWhyText(static_cast<dsa::AllocationWhy>(why));
        EXPECT(!text.empty() && text != "?");
    }
}

// nfr1: a line names the size, the memory type and its flags, what the memory is dedicated to
// and why.
static void nfr1_a_line_names_the_type_the_dedicated_target_and_why() {
    dsa::AllocationRecord record;
    record.bytes = 6684672;
    record.memoryType = 2;
    record.typeFlags = dsa::kTypeDeviceLocal | dsa::kTypeHostVisible | dsa::kTypeHostCached;
    record.dedicated = dsa::Dedicated::Buffer;
    record.why = dsa::AllocationWhy::RendererOwn;
    const std::string line = dsa::allocationColumns(record);
    EXPECT(contains(line, "6684672\t2\tdevice-local host-visible cached\tbuffer\t"));
    EXPECT(contains(line, "made by the renderer"));
    EXPECT(dsa::memoryTypeText(0) == "none");
    EXPECT(dsa::memoryTypeText(dsa::kTypeHostVisible | dsa::kTypeHostCoherent) ==
           "host-visible coherent");
    EXPECT(std::string(dsa::dedicatedText(dsa::Dedicated::Nothing)) == "-");
    EXPECT(std::string(dsa::dedicatedText(dsa::Dedicated::Image)) == "image");
}

// nfr1: a grant over 16 MiB is named again above the list with its process, size, memory type
// and age, so it can be matched with the guest's line that says what it is for; every line
// ends with its age.
static void nfr1_a_large_grant_is_named_with_its_age() {
    dsa::AllocationRecord grant;
    grant.bytes = 67174400;
    grant.memoryType = 2;
    grant.typeFlags = dsa::kTypeDeviceLocal | dsa::kTypeHostVisible | dsa::kTypeHostCoherent |
                      dsa::kTypeHostCached;
    grant.why = dsa::AllocationWhy::Granted;
    EXPECT(dsa::isLargeGrantRecord(grant));
    const std::string note = dsa::largeGrantNote("/system/bin/surfaceflinger (pid 500)", grant,
                                                 dsa::ageText(5000000, 1500000));
    EXPECT(note ==
           "# Large grant: /system/bin/surfaceflinger (pid 500), 67174400 bytes, memory type 2 "
           "(device-local host-visible coherent cached), made 3.5 s ago. The guest's log names "
           "what it is for (dsa-vk large grant).");
    dsa::AllocationRecord small = grant;
    small.bytes = 1114112;
    EXPECT(!dsa::isLargeGrantRecord(small));
    dsa::AllocationRecord gpuOnly = grant;
    gpuOnly.why = dsa::AllocationWhy::GpuOnly;
    EXPECT(!dsa::isLargeGrantRecord(gpuOnly));
    EXPECT(dsa::ageText(10, 0) == "-");
    EXPECT(dsa::ageText(10, 20) == "-");
    EXPECT(dsa::ageText(2000000, 1000000) == "1.0");
}

int main() {
    nfr1_a_large_grant_is_named_with_its_age();
    nfr1_an_allocation_without_a_note_is_the_renderers_own();
    nfr1_a_note_goes_to_the_next_allocation_only();
    nfr1_a_note_stays_on_its_thread();
    nfr1_the_silent_reasons_are_said();
    nfr1_a_line_names_the_type_the_dedicated_target_and_why();
    if (failures) {
        std::fprintf(stderr, "%d failures\n", failures);
        return 1;
    }
    std::puts("allocation notes: all passed");
    return 0;
}
