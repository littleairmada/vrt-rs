use nom::{number::streaming::be_u8, Err, IResult, Needed};

use crate::Error;

/// VRT Packet Trailer
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Trailer {
    /// Indicates whether or not the timestamp in teh IF Data packet is calibrated to an external reference.
    pub calibrated_time_enable: bool,
    /// Indicates whether or not the data in the packet is valid.
    pub valid_data_enable: bool,
    /// Indicates whether or not any phase-locked loops affecting the data are locked and stable.
    pub reference_lock_enable: bool,
    /// Indicates the AGC is active when true and indicates the MGC is active when false.
    pub agcmgc_enable: bool,
    /// Indicates whether or not the data contained in the packet containes some detected signal.
    pub detected_signal_enable: bool,
    /// Indicates whether or not the signal conveyed in the data payload has an inverted spectrum referenced to the spectrum of the signal at the system Reference Point.
    pub spectral_inversion_enable: bool,
    /// Indicates whether or not at least 1 data sample in the payload is invalid due to the signal exceeding the range of the data item.
    pub overrange_enable: bool,
    /// Indicates whether or not the packet contains at least one sample discontinuity due to processing errors and/or buffer overflow.
    pub sample_loss_enable: bool,
    /// Indicates whether or not user defined enable option 1 is set.
    pub user_defined_enable_1: bool,
    /// Indicates whether or not user defined enable option 2 is set.
    pub user_defined_enable_2: bool,
    /// Indicates whether or not user defined enable option 3 is set.
    pub user_defined_enable_3: bool,
    /// Indicates whether or not user defined enable option 4 is set.
    pub user_defined_enable_4: bool,
    /// To be documented later
    pub calibrated_time_indicator: bool,
    /// To be documented later
    pub valid_data_indicator: bool,
    /// To be documented later
    pub reference_lock_indicator: bool,
    /// To be documented later
    pub agcmgc_indicator: bool,
    /// To be documented later
    pub detected_signal_indicator: bool,
    /// To be documented later
    pub spectral_inversion_indicator: bool,
    /// To be documented later
    pub overrange_indicator: bool,
    /// To be documented later
    pub sample_loss_indicator: bool,
    /// To be documented later
    pub user_defined_indicator_1: bool,
    /// To be documented later
    pub user_defined_indicator_2: bool,
    /// To be documented later
    pub user_defined_indicator_3: bool,
    /// To be documented later
    pub user_defined_indicator_4: bool,
    /// Indicates whether or not the associated_context_packet_count is defined.
    pub associated_context_packet_count_enable: bool,
    /// A count of all transmitted context packets that are directly or indirectly associated with the IF Data packet or a count of some special subset of these.
    pub associated_context_packet_count: u8,
}

impl Trailer {
    /// Parse the VRT packet trailer
    pub fn parse(i: &[u8]) -> IResult<&[u8], Trailer> {
        if i.len() < 4 {
            return Err(Err::Incomplete(Needed::new(4)));
        }

        let (i, first_byte) = be_u8(i)?;
        let (i, second_byte) = be_u8(i)?;
        let (i, third_byte) = be_u8(i)?;
        let (i, fourth_byte) = be_u8(i)?;

        // first byte
        let calibrated_time_enable: u8 = (first_byte >> 7) & 0x01;
        let valid_data_enable: u8 = (first_byte >> 6) & 0x01;
        let reference_lock_enable: u8 = (first_byte >> 5) & 0x01;
        let agcmgc_enable: u8 = (first_byte >> 4) & 0x01;
        let detected_signal_enable: u8 = (first_byte >> 3) & 0x01;
        let spectral_inversion_enable: u8 = (first_byte >> 2) & 0x01;
        let overrange_enable: u8 = (first_byte >> 1) & 0x01;
        let sample_loss_enable: u8 = (first_byte) & 0x01;

        // second byte
        let user_defined_enable_1: u8 = (second_byte >> 7) & 0x01;
        let user_defined_enable_2: u8 = (second_byte >> 6) & 0x01;
        let user_defined_enable_3: u8 = (second_byte >> 5) & 0x01;
        let user_defined_enable_4: u8 = (second_byte >> 4) & 0x01;
        let calibrated_time_indicator: u8 = (second_byte >> 3) & 0x01;
        let valid_data_indicator: u8 = (second_byte >> 2) & 0x01;
        let reference_lock_indicator: u8 = (second_byte >> 1) & 0x01;
        let agcmgc_indicator: u8 = (second_byte) & 0x01;

        // third byte
        let detected_signal_indicator: u8 = (third_byte >> 7) & 0x01;
        let spectral_inversion_indicator: u8 = (third_byte >> 6) & 0x01;
        let overrange_indicator: u8 = (third_byte >> 5) & 0x01;
        let sample_loss_indicator: u8 = (third_byte >> 4) & 0x01;
        let user_defined_indicator_1: u8 = (third_byte >> 3) & 0x01;
        let user_defined_indicator_2: u8 = (third_byte >> 2) & 0x01;
        let user_defined_indicator_3: u8 = (third_byte >> 1) & 0x01;
        let user_defined_indicator_4: u8 = (third_byte) & 0x01;

        // fourth byte
        let associated_context_packet_count_enable: u8 = (fourth_byte >> 7) & 0x01;
        let associated_context_packet_count_value: u8 = (fourth_byte) & 0x7f;

        let hdr: Trailer = Trailer {
            calibrated_time_enable: calibrated_time_enable != 0,
            valid_data_enable: valid_data_enable != 0,
            reference_lock_enable: reference_lock_enable != 0,
            agcmgc_enable: agcmgc_enable != 0,
            detected_signal_enable: detected_signal_enable != 0,
            spectral_inversion_enable: spectral_inversion_enable != 0,
            overrange_enable: overrange_enable != 0,
            sample_loss_enable: sample_loss_enable != 0,
            user_defined_enable_1: user_defined_enable_1 != 0,
            user_defined_enable_2: user_defined_enable_2 != 0,
            user_defined_enable_3: user_defined_enable_3 != 0,
            user_defined_enable_4: user_defined_enable_4 != 0,
            calibrated_time_indicator: calibrated_time_indicator != 0,
            valid_data_indicator: valid_data_indicator != 0,
            reference_lock_indicator: reference_lock_indicator != 0,
            agcmgc_indicator: agcmgc_indicator != 0,
            detected_signal_indicator: detected_signal_indicator != 0,
            spectral_inversion_indicator: spectral_inversion_indicator != 0,
            overrange_indicator: overrange_indicator != 0,
            sample_loss_indicator: sample_loss_indicator != 0,
            user_defined_indicator_1: user_defined_indicator_1 != 0,
            user_defined_indicator_2: user_defined_indicator_2 != 0,
            user_defined_indicator_3: user_defined_indicator_3 != 0,
            user_defined_indicator_4: user_defined_indicator_4 != 0,
            associated_context_packet_count_enable: associated_context_packet_count_enable != 0,
            associated_context_packet_count: associated_context_packet_count_value,
        };
        Ok((i, hdr))
    }

    /// Serialize the VRT packet trailer
    pub fn serialize(&self, buffer: &mut [u8]) -> Result<usize, Error> {
        if buffer.len() < 4 {
            return Err(Error::BufferFull);
        }

        let mut word = 0;
        word |= u32::from(self.calibrated_time_enable) << 31;
        word |= u32::from(self.valid_data_enable) << 30;
        word |= u32::from(self.reference_lock_enable) << 29;
        word |= u32::from(self.agcmgc_enable) << 28;
        word |= u32::from(self.detected_signal_enable) << 27;
        word |= u32::from(self.spectral_inversion_enable) << 26;
        word |= u32::from(self.overrange_enable) << 25;
        word |= u32::from(self.sample_loss_enable) << 24;
        word |= u32::from(self.user_defined_enable_1) << 23;
        word |= u32::from(self.user_defined_enable_2) << 22;
        word |= u32::from(self.user_defined_enable_3) << 21;
        word |= u32::from(self.user_defined_enable_4) << 20;
        word |= u32::from(self.calibrated_time_indicator) << 19;
        word |= u32::from(self.calibrated_time_indicator) << 18;
        word |= u32::from(self.reference_lock_indicator) << 17;
        word |= u32::from(self.agcmgc_indicator) << 16;
        word |= u32::from(self.detected_signal_indicator) << 15;
        word |= u32::from(self.spectral_inversion_indicator) << 14;
        word |= u32::from(self.overrange_indicator) << 13;
        word |= u32::from(self.sample_loss_indicator) << 12;
        word |= u32::from(self.user_defined_indicator_1) << 11;
        word |= u32::from(self.user_defined_indicator_2) << 10;
        word |= u32::from(self.user_defined_indicator_3) << 9;
        word |= u32::from(self.user_defined_indicator_4) << 8;
        word |= u32::from(self.associated_context_packet_count_enable) << 7;
        word |= u32::from(self.associated_context_packet_count) & 0x7f;

        buffer.copy_from_slice(&word.to_be_bytes());

        Ok(4)
    }
}
