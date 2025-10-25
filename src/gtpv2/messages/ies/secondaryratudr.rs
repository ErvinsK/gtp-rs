// Secondary RAT Usage Data Report IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Secondary RAT Usage Data Report IE TV

pub const SCND_RAT_UDR: u8 = 201;
pub const SCND_RAT_UDR_LENGTH: usize = 27;

// Secondary RAT Usage Data Report IE implementation

// SRUDN (Secondary RAT Usage Data Report From NG-RAN): This bit indicates the presence of Length of Secondary RAT Data Usage Report Transfer and Secondary RAT Data Usage Report Transfer fields.
// IRSGW (Intended Receiver SGW): This bit defines if the Usage Data Report shall be used by the SGW or not. If set to 1 the SGW shall store it. If set to zero the SGW shall not store it.
// IRPGW (Intended Receiver PGW): This bit defines if the Usage Data Report shall be sent to the PGW or not. If set to 1 the SGW shall forward it to PGW and PGW shall store it. If set to zero SGW shall not forward it to PGW.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecondaryRatUsageDataReport {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub irsgw: bool,
    pub irpgw: bool,
    pub rat_type: u8,
    pub ebi: u8,
    pub start_timestamp: u32,
    pub end_timestamp: u32,
    pub usg_data_dl: u64,
    pub usg_data_ul: u64,
    pub second_rat_dur_tansfer: Option<Vec<u8>>, // Secondary RAT Data Usage Report Transfer
}

impl Default for SecondaryRatUsageDataReport {
    fn default() -> Self {
        SecondaryRatUsageDataReport {
            t: SCND_RAT_UDR,
            length: SCND_RAT_UDR_LENGTH as u16,
            ins: 0,
            irsgw: false,
            irpgw: false,
            rat_type: 0,
            ebi: 0,
            start_timestamp: 0,
            end_timestamp: 0,
            usg_data_dl: 0,
            usg_data_ul: 0,
            second_rat_dur_tansfer: None,
        }
    }
}

impl From<SecondaryRatUsageDataReport> for InformationElement {
    fn from(i: SecondaryRatUsageDataReport) -> Self {
        InformationElement::SecondaryRatUsageDataReport(i)
    }
}

impl IEs for SecondaryRatUsageDataReport {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(SCND_RAT_UDR);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let flag = if self.second_rat_dur_tansfer.is_some() {
            0x04 | ((self.irsgw as u8) << 1) | (self.irpgw as u8)
        } else {
            ((self.irsgw as u8) << 1) | (self.irpgw as u8)
        };
        buffer_ie.push(flag);
        buffer_ie.push(self.rat_type);
        buffer_ie.push(self.ebi);
        buffer_ie.extend_from_slice(&self.start_timestamp.to_be_bytes());
        buffer_ie.extend_from_slice(&self.end_timestamp.to_be_bytes());
        buffer_ie.extend_from_slice(&self.usg_data_dl.to_be_bytes());
        buffer_ie.extend_from_slice(&self.usg_data_ul.to_be_bytes());
        if let Some(transfer) = &self.second_rat_dur_tansfer {
            buffer_ie.push(transfer.len() as u8);
            buffer_ie.extend_from_slice(transfer);
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= SCND_RAT_UDR_LENGTH + MIN_IE_SIZE {
            let mut data = SecondaryRatUsageDataReport {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..SecondaryRatUsageDataReport::default()
            };
            match buffer[4] & 0x03 {
                0 => (data.irsgw, data.irpgw) = (false, false),
                1 => (data.irsgw, data.irpgw) = (false, true),
                2 => (data.irsgw, data.irpgw) = (true, false),
                3 => (data.irsgw, data.irpgw) = (true, true),
                _ => return Err(GTPV2Error::IEIncorrect(SCND_RAT_UDR)),
            }
            data.rat_type = buffer[5];
            data.ebi = buffer[6] & 0x0f;
            data.start_timestamp = u32::from_slice(&buffer[7..11]);
            data.end_timestamp = u32::from_slice(&buffer[11..15]);
            data.usg_data_dl = u64::from_slice(&buffer[15..23]);
            data.usg_data_ul = u64::from_slice(&buffer[23..31]);
            if buffer[4] & 0x04 == 0x04 {
                if buffer.len() > SCND_RAT_UDR_LENGTH + MIN_IE_SIZE {
                    let len = buffer[31] as usize;
                    if buffer.len() > SCND_RAT_UDR_LENGTH + MIN_IE_SIZE + len {
                        data.second_rat_dur_tansfer = Some(
                            buffer[(SCND_RAT_UDR_LENGTH + MIN_IE_SIZE + 1)
                                ..(SCND_RAT_UDR_LENGTH + MIN_IE_SIZE + 1 + len)]
                                .to_vec(),
                        );
                    } else {
                        return Err(GTPV2Error::IEIncorrect(SCND_RAT_UDR));
                    }
                } else {
                    return Err(GTPV2Error::IEIncorrect(SCND_RAT_UDR));
                }
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(SCND_RAT_UDR))
        }
    }

    fn len(&self) -> usize {
        (self.length as usize) + MIN_IE_SIZE
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }
    fn get_ins(&self) -> u8 {
        self.ins
    }
    fn get_type(&self) -> u8 {
        self.t
    }
}

#[test]
fn secondary_rat_udr_ie_unmarshal_test() {
    let encoded: [u8; 31] = [
        0xc9, 0x00, 0x1b, 0x00, 0x03, 0x00, 0x05, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0xff,
        0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff,
        0xff,
    ];
    let decoded = SecondaryRatUsageDataReport {
        t: SCND_RAT_UDR,
        length: SCND_RAT_UDR_LENGTH as u16,
        ins: 0,
        irsgw: true,
        irpgw: true,
        rat_type: 0,
        ebi: 5,
        start_timestamp: 0xff,
        end_timestamp: 0xffff,
        usg_data_dl: 0xffffff00,
        usg_data_ul: 0xffff,
        ..SecondaryRatUsageDataReport::default()
    };
    let i = SecondaryRatUsageDataReport::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn secondary_rat_udr_ie_marshal_test() {
    let encoded: [u8; 31] = [
        0xc9, 0x00, 0x1b, 0x00, 0x03, 0x00, 0x05, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0xff, 0xff,
        0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff,
        0xff,
    ];
    let decoded = SecondaryRatUsageDataReport {
        t: SCND_RAT_UDR,
        length: SCND_RAT_UDR_LENGTH as u16,
        ins: 0,
        irsgw: true,
        irpgw: true,
        rat_type: 0,
        ebi: 5,
        start_timestamp: 0xff,
        end_timestamp: 0xffff,
        usg_data_dl: 0xffffff00,
        usg_data_ul: 0xffff,
        ..SecondaryRatUsageDataReport::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
