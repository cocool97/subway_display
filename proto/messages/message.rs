use crate::{NetworkSendable, messages::MessageType};

pub struct Message<T: NetworkSendable> {
    pub message_type: MessageType,
    pub data: T,
}

impl<T: NetworkSendable> NetworkSendable for Message<T> {
    const OUT_SIZE: usize = MessageType::OUT_SIZE + T::OUT_SIZE;

    fn serialize(&self, out: &mut [u8]) {
        self.message_type
            .serialize(&mut out[..MessageType::OUT_SIZE]);
        self.data.serialize(&mut out[MessageType::OUT_SIZE..]);
    }

    fn deserialize(input: &[u8]) -> anyhow::Result<Self> {
        let message_type = MessageType::deserialize(&input[..MessageType::OUT_SIZE])?;
        let data = T::deserialize(&input[MessageType::OUT_SIZE..])?;
        Ok(Self { message_type, data })
    }
}
