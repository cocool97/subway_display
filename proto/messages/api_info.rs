use crate::NetworkSendable;

pub const NEXT_SUBWAY_ERROR_VALUE: i64 = 255;

/// Message representing the API information about subway arrivals.
#[derive(Clone, Default)]
#[repr(C, packed)]
pub struct ApiInfo {
    /// Interval in minutes from now before the next subway arrival.
    pub next_subway_interval_mins: u8,
    /// Interval in minutes from now before the second subway arrival.
    pub second_subway_interval_mins: u8,
}

impl NetworkSendable for ApiInfo {
    const OUT_SIZE: usize = core::mem::size_of::<Self>();

    fn serialize(&self, out: &mut [u8]) {
        out[0] = self.next_subway_interval_mins;
        out[1] = self.second_subway_interval_mins;
    }

    fn deserialize(input: &[u8]) -> anyhow::Result<Self> {
        Ok(Self {
            next_subway_interval_mins: input[0],
            second_subway_interval_mins: input[1],
        })
    }
}
