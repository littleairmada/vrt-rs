use crate::Error;

/// VRT Packet Type
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PktType {
    /// Standard IFDATA packet
    #[default]
    IfData,
    /// Standard IFDATA packet with a packet stream
    IfDataWithStream,
    /// Custom Extension Data packet
    ExtData,
    /// Custom Extension Data packet with a packet stream
    ExtDataWithStream,
    /// Standard IFCONTEXT packet with a context packet stream
    IfContext,
    /// Custom Extension packet with a context packet stream
    ExtContext,
}

impl TryFrom<u8> for PktType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PktType::IfData),
            1 => Ok(PktType::IfDataWithStream),
            2 => Ok(PktType::ExtData),
            3 => Ok(PktType::ExtDataWithStream),
            4 => Ok(PktType::IfContext),
            5 => Ok(PktType::ExtContext),
            _ => Err(Error::PktType(value)),
        }
    }
}

impl From<PktType> for u8 {
    fn from(pkt_type: PktType) -> u8 {
        match pkt_type {
            PktType::IfData => 0,
            PktType::IfDataWithStream => 1,
            PktType::ExtData => 2,
            PktType::ExtDataWithStream => 3,
            PktType::IfContext => 4,
            PktType::ExtContext => 5,
        }
    }
}
