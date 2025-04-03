use nom::{
    number::streaming::{be_u32, be_u64},
    Err, IResult, Needed,
};

use crate::Error;

use super::*;

/// VRT Packet
#[derive(Debug, Default, PartialEq)]
pub struct VrtPacket<'a> {
    /// VRT Packet Header
    pub header: Header,
    /// Optional Stream Id
    pub stream_id: Option<u32>,
    /// Optional Class Id
    pub class_id: Option<ClassId>,
    /// Optional Integer-Seconds Timestamp
    pub tsi: Option<u32>,
    /// Optional Fractional-Seconds Timestamp
    pub tsf: Option<u64>,
    /// Data Payload
    pub payload: &'a [u8],
    /// Optional VRT Packet Trailer
    pub trailer: Option<Trailer>,
}

impl VrtPacket<'_> {
    /// Parse the VRT packet
    pub fn parse(i: &[u8]) -> IResult<&[u8], VrtPacket<'_>> {
        let (i, header) = Header::parse(i)?;

        let expected_size = header.packet_size as usize * size_of::<u32>();
        let actual_size = i.len() + size_of::<u32>();
        if actual_size < expected_size {
            return Err(Err::Incomplete(Needed::new(expected_size)));
        }

        // Track the mandatory and optional fields to get the payload length
        let mut payload_len = expected_size;
        payload_len -= size_of::<u32>(); // header word
        if header.t {
            payload_len -= size_of::<u32>(); // trailer word
        }

        let (i, stream_id) = if matches!(
            header.packet_type,
            PktType::IfDataWithStream | PktType::ExtDataWithStream
        ) {
            let (i, stream_id) = be_u32(i)?;
            payload_len -= size_of_val(&stream_id);
            (i, Some(stream_id))
        } else {
            (i, None)
        };

        let (i, class_id) = if header.c {
            let (i, class_id) = ClassId::parse(i)?;
            payload_len -= size_of_val(&class_id);
            (i, Some(class_id))
        } else {
            (i, None)
        };

        let (i, tsi) = if header.tsi == Tsi::None {
            (i, None)
        } else {
            let (i, tsi) = be_u32(i)?;
            payload_len -= size_of_val(&tsi);
            (i, Some(tsi))
        };

        let (i, tsf) = if header.tsf == Tsf::None {
            (i, None)
        } else {
            let (i, tsf) = be_u64(i)?;
            payload_len -= size_of_val(&tsf);
            (i, Some(tsf))
        };

        let (data_payload, i) = i.split_at(payload_len);

        let (i, trailer) = if header.t {
            let (i, trailer) = Trailer::parse(i)?;
            (i, Some(trailer))
        } else {
            (i, None)
        };

        let packet = VrtPacket {
            header,
            stream_id,
            class_id,
            tsi,
            tsf,
            payload: data_payload,
            trailer,
        };

        Ok((i, packet))
    }

    /// Serialize the VITA-49 packet into the provided buffer.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer to serialize the packet into.
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - The number of bytes written to the buffer.
    /// * `Err(Error)` - An error if the buffer is too small or if serialization fails.
    ///
    /// # Example
    ///
    /// ```
    /// use vrt::VrtPacket;
    ///
    /// let mut packet = VrtPacket::default();
    /// // Set the fields of the packet as needed
    /// // packet.header.packet_type = ...;
    /// let mut buffer = [0u8; 1024]; // Ensure the buffer is large enough
    ///
    /// match packet.serialize(&mut buffer) {
    ///    Ok(size) => println!("Serialized {} bytes", size),
    ///    Err(e) => eprintln!("Error: {:?}", e),
    /// }
    /// ```
    pub fn serialize(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        let mut offset = 0;

        offset += self.header.serialize(&mut buffer[offset..])?;
        if let Some(stream_id) = self.stream_id {
            if buffer.len() < offset + size_of::<u32>() {
                return Err(Error::BufferFull);
            }
            buffer[offset..offset + size_of::<u32>()].copy_from_slice(&stream_id.to_be_bytes());
            offset += size_of::<u32>();
        }
        if let Some(class_id) = self.class_id {
            offset += class_id.serialize(&mut buffer[offset..])?;
        }
        if let Some(tsi) = self.tsi {
            if buffer.len() < offset + size_of_val(&tsi) {
                return Err(Error::BufferFull);
            }
            buffer[offset..offset + size_of_val(&tsi)].copy_from_slice(&tsi.to_be_bytes());
            offset += size_of_val(&tsi);
        }
        if let Some(tsf) = self.tsf {
            if buffer.len() < offset + size_of_val(&tsf) {
                return Err(Error::BufferFull);
            }
            buffer[offset..offset + size_of_val(&tsf)].copy_from_slice(&tsf.to_be_bytes());
            offset += size_of_val(&tsf);
        }
        if buffer.len() < offset + self.payload.len() {
            return Err(Error::BufferFull);
        }
        buffer[offset..offset + self.payload.len()].copy_from_slice(self.payload);
        offset += self.payload.len();
        if let Some(trailer) = self.trailer {
            offset += trailer.serialize(&mut buffer[offset..])?;
        }

        // Serialize the header again to update the packet size
        self.header.packet_size = (offset / size_of::<u32>()).try_into()?;
        let _ = self.header.serialize(&mut buffer[0..])?;

        Ok(offset)
    }
}
