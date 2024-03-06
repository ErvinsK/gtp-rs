// Group Id IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Group Id IE Type

pub const GROUP_ID: u8 = 216;

// Group Id IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupId {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub groupid: Vec<u8>,
}

impl Default for GroupId {
    fn default() -> Self {
        GroupId {
            t: GROUP_ID,
            length: 0,
            ins: 0,
            groupid: vec![],
        }
    }
}

impl From<GroupId> for InformationElement {
    fn from(i: GroupId) -> Self {
        InformationElement::GroupId(i)
    }
}

impl IEs for GroupId {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(GROUP_ID);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.extend_from_slice(&self.groupid[..]);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = GroupId {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..GroupId::default()
            };
            if check_tliv_ie_buffer(data.length, buffer) {
                data.groupid
                    .extend_from_slice(&buffer[4..((data.length as usize) + MIN_IE_SIZE)]);
                Ok(data)
            } else {
                Err(GTPV2Error::IEInvalidLength(GROUP_ID))
            }
        } else {
            Err(GTPV2Error::IEInvalidLength(GROUP_ID))
        }
    }

    fn len(&self) -> usize {
        self.length as usize + MIN_IE_SIZE
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
fn group_id_ie_marshal_test() {
    let encoded: [u8; 7] = [0xd8, 0x00, 0x03, 0x00, 0x0a, 0xff, 0x00];
    let decoded = GroupId {
        length: 3,
        groupid: vec![0x0a, 0xff, 0x00],
        ..GroupId::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn group_id_ie_unmarshal_test() {
    let encoded: [u8; 7] = [0xd8, 0x00, 0x03, 0x00, 0x0a, 0xff, 0x00];
    let decoded = GroupId {
        length: 3,
        groupid: vec![0x0a, 0xff, 0x00],
        ..GroupId::default()
    };
    assert_eq!(GroupId::unmarshal(&encoded).unwrap(), decoded);
}
