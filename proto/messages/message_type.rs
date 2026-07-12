use crate::NetworkSendable;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum MessageType {
    Time = 0,
    ApiInfo = 1,
}

impl NetworkSendable for MessageType {
    const OUT_SIZE: usize = 1;

    fn serialize(&self, out: &mut [u8]) {
        out[0] = *self as u8;
    }

    fn deserialize(input: &[u8]) -> anyhow::Result<Self> {
        match input[0] {
            0 => Ok(Self::Time),
            1 => Ok(Self::ApiInfo),
            _ => anyhow::bail!("Invalid message type"),
        }
    }
}
