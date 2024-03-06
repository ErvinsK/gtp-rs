// V2X Information IE (Grouped IE) - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{errors::GTPV2Error, messages::ies::*};

// V2X Information IE Type

pub const V2X_INFO: u8 = 208;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct V2xInformation {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub services_auth: Vec<ServicesAuthorized>,
    pub sidelink_max_brs: Vec<BitRate>, // Sidelink Max Bit Rates
    pub pc5_qos_params: Option<PC5QosParameters>,
}

impl Default for V2xInformation {
    fn default() -> Self {
        V2xInformation {
            t: V2X_INFO,
            length: 0,
            ins: 0,
            services_auth: Vec::new(),
            sidelink_max_brs: Vec::new(),
            pc5_qos_params: None,
        }
    }
}

impl From<V2xInformation> for GroupedIe {
    fn from(i: V2xInformation) -> Self {
        GroupedIe {
            t: V2X_INFO,
            length: i.length,
            ins: i.ins,
            elements: i.to_vec(),
        }
    }
}

impl From<GroupedIe> for V2xInformation {
    fn from(i: GroupedIe) -> Self {
        let mut data = V2xInformation::default();
        (data.t, data.length, data.ins) = (i.t, i.length, i.ins);
        for j in i.elements.into_iter() {
            match j {
                InformationElement::ServicesAuthorized(k) => {
                    if k.ins < 2 {
                        data.services_auth.push(k)
                    }
                }
                InformationElement::BitRate(k) => {
                    if k.ins < 2 {
                        data.sidelink_max_brs.push(k)
                    }
                }
                InformationElement::PC5QosParameters(k) => {
                    if let (0, false) = (k.ins, data.pc5_qos_params.is_some()) {
                        data.pc5_qos_params = Some(k)
                    }
                }
                _ => (),
            }
        }
        data
    }
}

impl IEs for V2xInformation {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let g_ie: GroupedIe = self.clone().into();
        g_ie.marshal(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let data: V2xInformation = match GroupedIe::unmarshal(buffer) {
            Ok(i) => V2xInformation::from(i),
            Err(j) => return Err(j),
        };
        Ok(data)
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

impl V2xInformation {
    fn to_vec(&self) -> Vec<InformationElement> {
        let mut v: Vec<InformationElement> = vec![];
        self.services_auth
            .iter()
            .for_each(|x| v.push(x.clone().into()));
        self.sidelink_max_brs
            .iter()
            .for_each(|x| v.push(x.clone().into()));
        if let Some(i) = self.pc5_qos_params.clone() {
            v.push(InformationElement::PC5QosParameters(i.clone()))
        }
        v
    }
}

#[test]
fn v2x_info_ie_unmarshal_test() {
    let encoded: [u8; 74] = [
        0xd0, 0x00, 0x46, 0x00, 0xd2, 0x00, 0x02, 0x00, 0x00, 0x01, 0xd2, 0x00, 0x02, 0x01, 0x00,
        0x01, 0xd3, 0x00, 0x04, 0x00, 0xaa, 0xff, 0xff, 0xff, 0xd3, 0x00, 0x04, 0x01, 0xff, 0xff,
        0xff, 0xaa, 0xd1, 0x00, 0x26, 0x00, 0xd4, 0x00, 0x0b, 0x00, 0x01, 0x05, 0x00, 0x00, 0xaa,
        0xaa, 0x00, 0x00, 0xff, 0xff, 0x01, 0xd4, 0x00, 0x0b, 0x00, 0x01, 0x06, 0x00, 0x00, 0xaa,
        0xaa, 0x00, 0x00, 0xff, 0xff, 0x01, 0xd3, 0x00, 0x04, 0x00, 0xff, 0xff, 0xff, 0xff,
    ];
    let decoded = V2xInformation {
        t: V2X_INFO,
        length: 70,
        ins: 0,
        services_auth: vec![
            ServicesAuthorized {
                t: SERVICES_AUTH,
                length: SERVICES_AUTH_LENGTH as u16,
                ins: 0,
                vehicle_ue_auth: false,
                pedestrian_ue_auth: true,
            },
            ServicesAuthorized {
                t: SERVICES_AUTH,
                length: SERVICES_AUTH_LENGTH as u16,
                ins: 1,
                vehicle_ue_auth: false,
                pedestrian_ue_auth: true,
            },
        ],
        sidelink_max_brs: vec![
            BitRate {
                bitrate: 0xaaffffff,
                ..BitRate::default()
            },
            BitRate {
                ins: 1,
                bitrate: 0xffffffaa,
                ..BitRate::default()
            },
        ],
        pc5_qos_params: Some(PC5QosParameters {
            t: PC5_QOS_PARAM,
            length: 38,
            ins: 0,
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
        }),
    };
    let i = V2xInformation::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn v2x_info_ie_marshal_test() {
    let encoded: [u8; 74] = [
        0xd0, 0x00, 0x46, 0x00, 0xd2, 0x00, 0x02, 0x00, 0x00, 0x01, 0xd2, 0x00, 0x02, 0x01, 0x00,
        0x01, 0xd3, 0x00, 0x04, 0x00, 0xaa, 0xff, 0xff, 0xff, 0xd3, 0x00, 0x04, 0x01, 0xff, 0xff,
        0xff, 0xaa, 0xd1, 0x00, 0x26, 0x00, 0xd4, 0x00, 0x0b, 0x00, 0x01, 0x05, 0x00, 0x00, 0xaa,
        0xaa, 0x00, 0x00, 0xff, 0xff, 0x01, 0xd4, 0x00, 0x0b, 0x00, 0x01, 0x06, 0x00, 0x00, 0xaa,
        0xaa, 0x00, 0x00, 0xff, 0xff, 0x01, 0xd3, 0x00, 0x04, 0x00, 0xff, 0xff, 0xff, 0xff,
    ];
    let decoded = V2xInformation {
        t: V2X_INFO,
        length: 70,
        ins: 0,
        services_auth: vec![
            ServicesAuthorized {
                t: SERVICES_AUTH,
                length: SERVICES_AUTH_LENGTH as u16,
                ins: 0,
                vehicle_ue_auth: false,
                pedestrian_ue_auth: true,
            },
            ServicesAuthorized {
                t: SERVICES_AUTH,
                length: SERVICES_AUTH_LENGTH as u16,
                ins: 1,
                vehicle_ue_auth: false,
                pedestrian_ue_auth: true,
            },
        ],
        sidelink_max_brs: vec![
            BitRate {
                bitrate: 0xaaffffff,
                ..BitRate::default()
            },
            BitRate {
                ins: 1,
                bitrate: 0xffffffaa,
                ..BitRate::default()
            },
        ],
        pc5_qos_params: Some(PC5QosParameters {
            t: PC5_QOS_PARAM,
            length: 38,
            ins: 0,
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
        }),
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded);
}
