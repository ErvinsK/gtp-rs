// PC5 QoS Parameters IE (Grouped IE) - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{errors::GTPV2Error, messages::ies::*};

// PC5 QoS Parameters IE Type

pub const PC5_QOS_PARAM: u8 = 209;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PC5QosParameters {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub pc5_qos_flows: Vec<PC5QosFlow>,
    pub pc5_link_abrs: Option<BitRate>, // PC5 Link Aggregated Bit Rates
}

impl Default for PC5QosParameters {
    fn default() -> Self {
        PC5QosParameters {
            t: PC5_QOS_PARAM,
            length: 0,
            ins: 0,
            pc5_qos_flows: Vec::new(),
            pc5_link_abrs: None,
        }
    }
}

impl From<PC5QosParameters> for GroupedIe {
    fn from(i: PC5QosParameters) -> Self {
        GroupedIe {
            t: PC5_QOS_PARAM,
            length: i.length,
            ins: i.ins,
            elements: i.to_vec(),
        }
    }
}

impl From<GroupedIe> for PC5QosParameters {
    fn from(i: GroupedIe) -> Self {
        let mut data = PC5QosParameters::default();
        (data.t, data.length, data.ins) = (i.t, i.length, i.ins);
        for j in i.elements.into_iter() {
            match j {
                InformationElement::PC5QosFlow(k) => {
                    if k.ins == 0 {
                        data.pc5_qos_flows.push(k)
                    }
                }
                InformationElement::BitRate(k) => {
                    if let (0, false) = (k.ins, data.pc5_link_abrs.is_some()) {
                        data.pc5_link_abrs = Some(k)
                    }
                }
                _ => (),
            }
        }
        data
    }
}

impl IEs for PC5QosParameters {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let g_ie: GroupedIe = self.clone().into();
        g_ie.marshal(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let data: PC5QosParameters = match GroupedIe::unmarshal(buffer) {
            Ok(i) => PC5QosParameters::from(i),
            Err(j) => return Err(j),
        };
        if data.pc5_qos_flows.is_empty() {
            Err(GTPV2Error::IEIncorrect(PC5_QOS_PARAM))
        } else {
            Ok(data)
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

impl PC5QosParameters {
    fn to_vec(&self) -> Vec<InformationElement> {
        let mut v: Vec<InformationElement> = vec![];
        self.pc5_qos_flows
            .iter()
            .for_each(|x| v.push(x.clone().into()));
        if let Some(i) = self.pc5_link_abrs.clone() {
            v.push(i.into())
        }
        v
    }
}

#[test]
fn pc5_qos_params_ie_unmarshal_test() {
    let encoded: [u8; 42] = [
        0xd1, 0x00, 0x26, 0x01, 0xd4, 0x00, 0x0b, 0x00, 0x01, 0x05, 0x00, 0x00, 0xaa, 0xaa, 0x00,
        0x00, 0xff, 0xff, 0x01, 0xd4, 0x00, 0x0b, 0x00, 0x01, 0x06, 0x00, 0x00, 0xaa, 0xaa, 0x00,
        0x00, 0xff, 0xff, 0x01, 0xd3, 0x00, 0x04, 0x00, 0xff, 0xff, 0xff, 0xff,
    ];
    let decoded = PC5QosParameters {
        t: PC5_QOS_PARAM,
        length: 38,
        ins: 1,
        pc5_qos_flows: vec![
            PC5QosFlow {
                length: 11,
                pqi_label: 5,
                gfbr: 0xaaaa,
                mfbr: 0xffff,
                range: Some(0x01),
                ..PC5QosFlow::default()
            },
            PC5QosFlow {
                length: 11,
                pqi_label: 6,
                gfbr: 0xaaaa,
                mfbr: 0xffff,
                range: Some(0x01),
                ..PC5QosFlow::default()
            },
        ],
        pc5_link_abrs: Some(BitRate {
            bitrate: 0xffffffff,
            ..BitRate::default()
        }),
    };
    let i = PC5QosParameters::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn pc5_qos_params_ie_marshal_test() {
    let encoded: [u8; 42] = [
        0xd1, 0x00, 0x26, 0x01, 0xd4, 0x00, 0x0b, 0x00, 0x01, 0x05, 0x00, 0x00, 0xaa, 0xaa, 0x00,
        0x00, 0xff, 0xff, 0x01, 0xd4, 0x00, 0x0b, 0x00, 0x01, 0x06, 0x00, 0x00, 0xaa, 0xaa, 0x00,
        0x00, 0xff, 0xff, 0x01, 0xd3, 0x00, 0x04, 0x00, 0xff, 0xff, 0xff, 0xff,
    ];
    let decoded = PC5QosParameters {
        t: PC5_QOS_PARAM,
        length: 38,
        ins: 1,
        pc5_qos_flows: vec![
            PC5QosFlow {
                length: 11,
                pqi_label: 5,
                gfbr: 0xaaaa,
                mfbr: 0xffff,
                range: Some(0x01),
                ..PC5QosFlow::default()
            },
            PC5QosFlow {
                length: 11,
                pqi_label: 6,
                gfbr: 0xaaaa,
                mfbr: 0xffff,
                range: Some(0x01),
                ..PC5QosFlow::default()
            },
        ],
        pc5_link_abrs: Some(BitRate {
            bitrate: 0xffffffff,
            ..BitRate::default()
        }),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded);
}
