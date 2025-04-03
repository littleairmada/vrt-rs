use crate::Error;

/// Integer-Seconds Timestamp
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Tsi {
    /// No integer-seconds timestamp field.
    #[default]
    None,
    /// Integer-seconds timestamp field is a Coordinated Universal Time (UTC).
    Utc,
    /// Integer-seconds timestamp field is a GPS time.
    Gps,
    /// Integer-seconds timestamp field is in another format.
    Other,
}

impl TryFrom<u8> for Tsi {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Tsi::None),
            1 => Ok(Tsi::Utc),
            2 => Ok(Tsi::Gps),
            3 => Ok(Tsi::Other),
            _ => Err(Error::Tsi(value)),
        }
    }
}

impl From<Tsi> for u8 {
    fn from(tsi: Tsi) -> u8 {
        match tsi {
            Tsi::None => 0,
            Tsi::Utc => 1,
            Tsi::Gps => 2,
            Tsi::Other => 3,
        }
    }
}
