// Copyright 2026 The DSA Authors
// SPDX-License-Identifier: Apache-2.0

#include "host_link.h"

#include <sys/socket.h>
// After sys/socket.h, which defines what it uses.
#include <linux/vm_sockets.h>
#include <sys/time.h>
#include <unistd.h>

#include <cerrno>
#include <utility>

#include "log.h"
#include "picture_region.h"

namespace dsa::codec2 {

namespace {

bool writeAll(int fd, const std::vector<uint8_t>& bytes) {
    size_t done = 0;
    while (done < bytes.size()) {
        const ssize_t n = send(fd, bytes.data() + done, bytes.size() - done, MSG_NOSIGNAL);
        if (n < 0 && errno == EINTR) continue;
        if (n <= 0) return false;
        done += static_cast<size_t>(n);
    }
    return true;
}

void setTimeout(int fd, int seconds) {
    timeval timeout{seconds, 0};
    setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &timeout, sizeof(timeout));
    setsockopt(fd, SOL_SOCKET, SO_SNDTIMEO, &timeout, sizeof(timeout));
}

// Connects to the video port and does the handshake; the host's capabilities come next.
// Returns the socket, or -1 with the reason in `error`.
int connectAndGreet(std::string* error, Capabilities* caps) {
    const int fd = socket(AF_VSOCK, SOCK_STREAM | SOCK_CLOEXEC, 0);
    if (fd < 0) {
        *error = "cannot make a vsock socket";
        return -1;
    }
    sockaddr_vm address{};
    address.svm_family = AF_VSOCK;
    address.svm_cid = VMADDR_CID_HOST;
    address.svm_port = kVideoPort;
    if (connect(fd, reinterpret_cast<const sockaddr*>(&address), sizeof(address)) != 0) {
        *error = "the host's video port is not open (is the DSA window manager running?)";
        ::close(fd);
        return -1;
    }
    setTimeout(fd, 5);
    std::vector<uint8_t> frame;
    if (!writeAll(fd, withLength(encodeHello("dsa-codec2"))) || !readHostFrame(fd, &frame)) {
        *error = "the host did not answer on the video port";
        ::close(fd);
        return -1;
    }
    const HostMessage hello = decodeHostFrame(frame.data(), frame.size());
    if (const auto* rejected = std::get_if<Rejected>(&hello)) {
        *error = "the host refused the video port: " + rejected->detail;
        ::close(fd);
        return -1;
    }
    if (!std::holds_alternative<Accepted>(hello) || !readHostFrame(fd, &frame)) {
        *error = "the host answered the video port's hello with something else";
        ::close(fd);
        return -1;
    }
    const HostMessage offered = decodeHostFrame(frame.data(), frame.size());
    const auto* got = std::get_if<Capabilities>(&offered);
    if (got == nullptr) {
        *error = "the host did not say what it decodes";
        ::close(fd);
        return -1;
    }
    *caps = *got;
    return fd;
}

}  // namespace

std::optional<std::vector<CodecSupport>> queryCapabilities() {
    std::string error;
    Capabilities caps;
    const int fd = connectAndGreet(&error, &caps);
    if (fd < 0) {
        CODEC_LOGW("%s", error.c_str());
        return std::nullopt;
    }
    ::close(fd);
    return caps.codecs;
}

std::unique_ptr<HostDecoder> HostDecoder::open(Codec codec, uint32_t width, uint32_t height,
                                               MessageHandler onMessage, std::string* error) {
    Capabilities caps;
    const int fd = connectAndGreet(error, &caps);
    if (fd < 0) {
        return nullptr;
    }
    std::vector<uint8_t> frame;
    // The id of the memory shared with the host, which this process maps: the host sends the
    // stream's pictures there when it writes to the same memory.
    const uint64_t region = PictureRegion::instance().id();
    if (!writeAll(fd, withLength(encodeOpen(codec, width, height, region))) || !readHostFrame(fd, &frame)) {
        *error = "the host did not answer the request for a decoder";
        ::close(fd);
        return nullptr;
    }
    const HostMessage answer = decodeHostFrame(frame.data(), frame.size());
    const auto* opened = std::get_if<Opened>(&answer);
    if (opened == nullptr || !opened->ok) {
        *error = opened ? opened->error : "the host answered the request for a decoder with something else";
        ::close(fd);
        return nullptr;
    }
    // Pictures come when the decoder has them; the reader waits as long as it takes. The writer
    // too: the host reads no packet while the guest has not taken its last picture, which
    // waits for a buffer of the app's surface. A send that timed out here would end the
    // stream on a pause, and close() ends both waits.
    timeval none{0, 0};
    setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &none, sizeof(none));
    setsockopt(fd, SOL_SOCKET, SO_SNDTIMEO, &none, sizeof(none));
    const bool shared = opened->sharedPictures;
    if (region != 0 && !shared) {
        CODEC_LOGW("the host sends this stream's pictures through the video port, though shared memory is mapped");
    }
    std::unique_ptr<HostDecoder> decoder = adopt(fd, std::move(onMessage));
    decoder->mSharedPictures = shared;
    return decoder;
}

void HostDecoder::SlotReturns::giveBack(uint32_t slot) {
    std::lock_guard<std::mutex> guard(lock);
    // Never dropped by a flush: a slot the host does not get back is one fewer for the stream.
    if (link != nullptr) {
        link->send(encodeSlotReturned(slot), false, std::chrono::milliseconds(100));
    }
}

MessageReader::Handler withSharedPictures(MessageReader::Handler onMessage,
                                          std::shared_ptr<HostDecoder::SlotReturns> returns) {
    return [onMessage = std::move(onMessage), returns = std::move(returns)](HostMessage message) {
        if (auto* frame = std::get_if<Frame>(&message); frame != nullptr && frame->slot != 0) {
            auto picture = std::make_shared<SharedPicture>();
            const uint64_t rows = uint64_t{frame->height} + (uint64_t{frame->height} + 1) / 2;
            const size_t length = static_cast<size_t>(uint64_t{frame->stride} * rows);
            // A slot outside the memory has no bytes: the picture is not written, and its slot
            // still goes back.
            picture->data = PictureRegion::instance().slice(frame->slotOffset, length);
            picture->size = picture->data != nullptr ? length : 0;
            const uint32_t slot = frame->slot;
            picture->giveBack = [returns, slot] { returns->giveBack(slot); };
            frame->shared = std::move(picture);
        }
        onMessage(std::move(message));
    };
}

std::unique_ptr<HostDecoder> HostDecoder::adopt(int fd, MessageHandler onMessage) {
    std::unique_ptr<HostDecoder> decoder(new HostDecoder(fd, std::move(onMessage)));
    decoder->mWriter = std::thread([raw = decoder.get()] { raw->write(); });
    return decoder;
}

// What may wait unsent: packets are compressed, and the framework keeps only a pipeline's worth
// of works in flight, so this is reached only when the host has stopped reading.
constexpr size_t kMaxQueuedBytes = 16 * 1024 * 1024;

HostDecoder::HostDecoder(int fd, MessageHandler onMessage)
    : mFd(fd),
      mOutgoing(kMaxQueuedBytes),
      mReturns(std::make_shared<SlotReturns>(this)),
      mReader(std::make_unique<MessageReader>(fd, withSharedPictures(std::move(onMessage), mReturns))) {}

HostDecoder::~HostDecoder() { close(); }

PushResult HostDecoder::send(const std::vector<uint8_t>& frame, bool droppable, std::chrono::milliseconds wait) {
    if (mClosed.load() || mWriteFailed.load()) {
        return PushResult::Closed;
    }
    return mOutgoing.pushFor(withLength(frame), droppable, wait);
}

bool HostDecoder::sendFlush(const std::vector<uint8_t>& frame) {
    if (mClosed.load() || mWriteFailed.load()) {
        return false;
    }
    return mOutgoing.pushReplacingDroppable(withLength(frame)) >= 0;
}

void HostDecoder::write() {
    std::vector<uint8_t> frame;
    while (mOutgoing.pop(&frame)) {
        if (!writeAll(mFd, frame)) {
            if (!mClosed.load()) {
                CODEC_LOGW("a packet did not reach the host's decoder");
            }
            mWriteFailed.store(true);
            mOutgoing.close();
            return;
        }
    }
}

void HostDecoder::close() {
    if (mClosed.exchange(true)) {
        return;
    }
    // A picture that outlives the link gives its slot back to no one: the host takes every
    // slot of the connection back as it ends.
    {
        std::lock_guard<std::mutex> guard(mReturns->lock);
        mReturns->link = nullptr;
    }
    // Nothing more is handed on, not even the socket's end as the host going away.
    if (mReader) {
        mReader->silence();
    }
    // Wakes the writer whether it waits for a packet or for the socket, and the reader.
    mOutgoing.close();
    shutdown(mFd, SHUT_RDWR);
    // The writer never closes the link itself, so it is never this thread.
    if (mWriter.joinable()) {
        if (mWriter.get_id() == std::this_thread::get_id()) {
            mWriter.detach();
        } else {
            mWriter.join();
        }
    }
    // Waits for the reader, unless this is the reader, inside its handler: it then ends when
    // the handler returns, without touching the socket again.
    if (mReader) {
        mReader->stop();
    }
    ::close(mFd);
    mFd = -1;
}

}  // namespace dsa::codec2
