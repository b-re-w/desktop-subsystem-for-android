//! What the host machine says about itself, which every host reads its own way.

/// Facts about the computer the host runs on.
pub trait HostFacts: Send + Sync {
    /// The computer's physical memory, in KiB; `None` when it cannot be read.
    fn physical_memory_kb(&self) -> Option<u64>;
}
