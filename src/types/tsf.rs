use crate::Error;

/// Fractional-Seconds Timestamp Type
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Tsf {
    /// No fractional-seconds timestamp field.
    #[default]
    None,
    /// Fractional-seconds timestamp field is a Sample Count Timestamp.
    SampleCount,
    /// Fractional-seconds timestamp field is a Real Time (Picoseconds) Timestamp.
    RealTime,
    /// Fractional-seconds timestamp field is a Free Running Count Timestamp.
    FreeRunning,
}

impl TryFrom<u8> for Tsf {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Tsf::None),
            1 => Ok(Tsf::SampleCount),
            2 => Ok(Tsf::RealTime),
            3 => Ok(Tsf::FreeRunning),
            _ => Err(Error::Tsf(value)),
        }
    }
}

impl From<Tsf> for u8 {
    fn from(tsf: Tsf) -> u8 {
        match tsf {
            Tsf::None => 0,
            Tsf::SampleCount => 1,
            Tsf::RealTime => 2,
            Tsf::FreeRunning => 3,
        }
    }
}
