#![no_std]

pub mod messages;

/// Trait for types that can be serialized and deserialized over the network.
pub trait NetworkSendable: Sized {
    const OUT_SIZE: usize;

    /// Serializes the type into the given buffer.
    /// `out` must be at least `OUT_SIZE` bytes long.
    fn serialize(&self, out: &mut [u8]);

    /// Deserializes the type from the given buffer.
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer is not `OUT_SIZE` bytes long.
    fn deserialize(input: &[u8]) -> anyhow::Result<Self>;
}
