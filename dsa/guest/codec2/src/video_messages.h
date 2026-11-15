// Copyright 2026 The DSA Authors
// SPDX-License-Identifier: Apache-2.0
//
// The video port's messages, through the codec generated from dsa/protocol/control.proto.
// Each frame is one encoded Envelope; on the socket each goes with its length in front.

#pragma once

#include <cstddef>
#include <cstdint>
#include <functional>
#include <memory>
#include <string>
#include <variant>
#include <vector>

namespace dsa::codec2 {

// A coding format, as the protocol numbers them.
enum class Codec : int32_t { H264 = 1, Hevc = 2, Vp9 = 3, Av1 = 4 };

struct CodecSupport {
    Codec codec = Codec::H264;
    uint32_t maxWidth = 0;
    uint32_t maxHeight = 0;
};

// What the host sends on the video port.
struct Accepted {};
struct Rejected {
    std::string detail;
};
struct Capabilities {
    std::vector<CodecSupport> codecs;
};
struct Opened {
    bool ok = false;
    std::string error;
    // Whether the stream's pictures may come in the memory shared with the host.
    bool sharedPictures = false;
};
// A picture in a slot of the memory shared with the host (picture_region.h): its bytes, read
// only, and what gives the slot back to the host once the last copy of the frame is gone, so a
// slot comes back however its picture ends (written, dropped by a flush, or never taken).
struct SharedPicture {
    const uint8_t* data = nullptr;
    size_t size = 0;
    std::function<void()> giveBack;
    SharedPicture() = default;
    SharedPicture(const SharedPicture&) = delete;
    SharedPicture& operator=(const SharedPicture&) = delete;
    ~SharedPicture() {
        if (giveBack) giveBack();
    }
};
struct Frame {
    uint64_t frameIndex = 0;
    int64_t timestampUs = 0;
    uint32_t width = 0;
    uint32_t height = 0;
    uint32_t stride = 0;
    std::string nv12;
    bool endOfStream = false;
    // Not 0: the picture is in this slot of the shared memory, `slotOffset` bytes in, and not
    // in `nv12`. The link fills in `shared` with its bytes.
    uint32_t slot = 0;
    uint64_t slotOffset = 0;
    std::shared_ptr<const SharedPicture> shared;
};
// The picture's bytes, wherever they came: in the message or in the shared memory. A picture
// whose slot is not inside the memory has none.
inline const uint8_t* pictureBytes(const Frame& frame) {
    if (frame.slot != 0) return frame.shared ? frame.shared->data : nullptr;
    return reinterpret_cast<const uint8_t*>(frame.nv12.data());
}
inline size_t pictureSize(const Frame& frame) {
    if (frame.slot != 0) return frame.shared && frame.shared->data ? frame.shared->size : 0;
    return frame.nv12.size();
}
// Whether `frame` says its packet gives no picture: the host did not decode it, since it came
// after a flush or the stream's start and before a key frame (its picture would be green).
// Such a frame has no size and no planes, and does not end the stream.
inline bool carriesNoPicture(const Frame& frame) {
    return !frame.endOfStream && frame.width == 0 && frame.height == 0 && frame.nv12.empty() && frame.slot == 0;
}

struct Flushed {};
struct DecodeError {
    std::string error;
};

using HostMessage =
    std::variant<std::monostate, Accepted, Rejected, Capabilities, Opened, Frame, Flushed, DecodeError>;

// What a frame from the host says; std::monostate for one that cannot be decoded or that the
// video port does not carry.
HostMessage decodeHostFrame(const uint8_t* data, size_t size);

// The codec HAL's Hello on the video port.
std::vector<uint8_t> encodeHello(const std::string& build);
// `pictureRegion` is the id of the shared picture memory this process maps, 0 for none.
std::vector<uint8_t> encodeOpen(Codec codec, uint32_t width, uint32_t height, uint64_t pictureRegion = 0);
// The guest is done with the picture in `slot`: the host may write it again.
std::vector<uint8_t> encodeSlotReturned(uint32_t slot);
std::vector<uint8_t> encodePacket(uint64_t frameIndex, int64_t timestampUs, const uint8_t* data, size_t size,
                                  bool codecConfig, bool endOfStream);
std::vector<uint8_t> encodeFlush();

// `frame` with its length in front, as the socket carries it.
std::vector<uint8_t> withLength(const std::vector<uint8_t>& frame);

}  // namespace dsa::codec2
