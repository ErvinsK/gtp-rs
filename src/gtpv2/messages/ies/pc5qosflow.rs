// PC5 QoS Flow IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{errors::GTPV2Error, messages::ies::*, utils::*};

// PC5 QoS Flow IE Type

pub const PC5_QOS_FLOW: u8 = 212;

// PC5 QoS Flow IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PC5QosFlow {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub pqi_label: u8,
    pub gfbr: u32, // Guaranteed Flow Bit Rate
    pub mfbr: u32, // Maximum Flow Bit Rate
    pub range: Option<u8>,
}

impl Default for PC5QosFlow {
    fn default() -> Self {
        PC5QosFlow {
            t: PC5_QOS_FLOW,
            length: 0,
            ins: 0,
            pqi_label: 0,
            gfbr: 0,
            mfbr: 0,
            range: None,
        }
    }
}

impl From<PC5QosFlow> for InformationElement {
    fn from(i: PC5QosFlow) -> Self {
        InformationElement::PC5QosFlow(i)
    }
}

impl IEs for PC5QosFlow {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(PC5_QOS_FLOW);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.range.is_some() as u8);
        buffer_ie.push(self.pqi_label);
        buffer_ie.extend_from_slice(&self.gfbr.to_be_bytes());
        buffer_ie.extend_from_slice(&self.mfbr.to_be_bytes());
        if let Some(i) = self.range {
            buffer_ie.push(i);
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = PC5QosFlow {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..PC5QosFlow::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                data.pqi_label = buffer[5];
                data.gfbr = u32::from_be_bytes([buffer[6], buffer[7], buffer[8], buffer[9]]);
                data.mfbr = u32::from_be_bytes([buffer[10], buffer[11], buffer[12], buffer[13]]);
                if buffer[4] & 0x01 == 1 {
                    data.range = Some(buffer[14]);
                }
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(PC5_QOS_FLOW))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(PC5_QOS_FLOW))
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
fn pc5_qos_flow_ie_unmarshal_test() {
    let encoded: [u8; 15] = [
        0xd4, 0x00, 0x0b, 0x00, 0x01, 0x05, 0x00, 0x00, 0xaa, 0xaa, 0x00, 0x00, 0xff, 0xff, 0x01,
    ];
    let decoded = PC5QosFlow {
        length: 11,
        pqi_label: 5,
        gfbr: 0xaaaa,
        mfbr: 0xffff,
        range: Some(0x01),
        ..PC5QosFlow::default()
    };
    let i = PC5QosFlow::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn pc5_qos_flow_ie_marshal_test() {
    let encoded: [u8; 15] = [
        0xd4, 0x00, 0x0b, 0x00, 0x01, 0x05, 0x00, 0x00, 0xaa, 0xaa, 0x00, 0x00, 0xff, 0xff, 0x01,
    ];
    let decoded = PC5QosFlow {
        length: 11,
        pqi_label: 5,
        gfbr: 0xaaaa,
        mfbr: 0xffff,
        range: Some(0x01),
        ..PC5QosFlow::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded);
}
