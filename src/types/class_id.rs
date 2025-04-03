use nom::{
    number::streaming::{be_u16, be_u32},
    IResult,
};

use crate::Error;

/// Class Identifier
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClassId {
    /// Organizationally Unique Identifier assigned by IEEE, VITA, the VRT Profile author, or a reserved OUI.
    pub oui: u32,
    /// Information Class Code
    pub information_class_code: u16,
    /// Packet Class Code
    pub packet_class_code: u16,
}

impl ClassId {
    /// Parse the Class ID
    pub fn parse(i: &[u8]) -> IResult<&[u8], ClassId> {
        let (i, oui) = be_u32(i)?;
        let (i, information_class_code) = be_u16(i)?;
        let (i, packet_class_code) = be_u16(i)?;

        Ok((
            i,
            ClassId {
                oui,
                information_class_code,
                packet_class_code,
            },
        ))
    }

    /// Serialize the Class ID
    pub fn serialize(&self, buffer: &mut [u8]) -> Result<usize, Error> {
        if buffer.len() < 8 {
            return Err(Error::BufferFull);
        }

        let mut word = u64::from(self.oui) << 32;
        word |= u64::from(self.information_class_code) << 16;
        word |= u64::from(self.packet_class_code);
        buffer[0..8].copy_from_slice(&word.to_be_bytes());

        Ok(8)
    }
}
