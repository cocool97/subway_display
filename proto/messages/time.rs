use crate::NetworkSendable;

/// Message representing a time.
/// It only holds the current hour as this is the only thing needed by the display.
#[derive(Clone, Default)]
#[repr(C, packed)]
pub struct Time(pub u8);

impl NetworkSendable for Time {
    const OUT_SIZE: usize = 1;

    fn serialize(&self, buf: &mut [u8]) {
        buf[0] = self.0;
    }

    fn deserialize(input: &[u8]) -> anyhow::Result<Self> {
        Ok(Self(input[0]))
    }
}
