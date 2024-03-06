// Bearer Context IE (Grouped IE) - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{errors::GTPV2Error, messages::ies::*};

// Bearer Context IE Type

pub const SCEF_PDN_CONN: u8 = 195;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScefPdnConnections {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub apn: Apn,
    pub default_ebi: Ebi,
    pub scef_id: NodeIdentifier,
}

impl Default for ScefPdnConnections {
    fn default() -> Self {
        ScefPdnConnections {
            t: SCEF_PDN_CONN,
            length: 0,
            ins: 0,
            apn: Apn::default(),
            default_ebi: Ebi::default(),
            scef_id: NodeIdentifier::default(),
        }
    }
}

impl From<ScefPdnConnections> for GroupedIe {
    fn from(i: ScefPdnConnections) -> Self {
        GroupedIe {
            t: SCEF_PDN_CONN,
            length: i.length,
            ins: i.ins,
            elements: i.to_vec(),
        }
    }
}

impl From<GroupedIe> for ScefPdnConnections {
    fn from(i: GroupedIe) -> Self {
        let mut pdn_conn = ScefPdnConnections::default();
        (pdn_conn.t, pdn_conn.length, pdn_conn.ins) = (i.t, i.length, i.ins);
        for j in i.elements.into_iter() {
            match j {
                InformationElement::Apn(k) => {
                    if k.ins == 0 {
                        pdn_conn.apn = k;
                    }
                }
                InformationElement::Ebi(k) => {
                    if k.ins == 0 {
                        pdn_conn.default_ebi = k;
                    }
                }
                InformationElement::NodeIdentifier(k) => {
                    if k.ins == 0 {
                        pdn_conn.scef_id = k;
                    }
                }
                _ => (),
            }
        }
        pdn_conn
    }
}

impl IEs for ScefPdnConnections {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let g_ie = GroupedIe::from(self.clone());
        g_ie.marshal(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let data: ScefPdnConnections = match GroupedIe::unmarshal(buffer) {
            Ok(i) => ScefPdnConnections::from(i),
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

impl ScefPdnConnections {
    fn to_vec(&self) -> Vec<InformationElement> {
        vec![
            self.apn.clone().into(),
            self.default_ebi.clone().into(),
            self.scef_id.clone().into(),
        ]
    }
}

#[test]
fn scef_pdn_connection_ie_unmarshal_test() {
    let encoded: [u8; 83] = [
        0xc3, 0x00, 0x4f, 0x00, 0x47, 0x00, 0x0d, 0x00, 0x04, 0x74, 0x65, 0x73, 0x74, 0x03, 0x6e,
        0x65, 0x74, 0x03, 0x63, 0x6f, 0x6d, 0x49, 0x00, 0x01, 0x00, 0x05, 0xb0, 0x00, 0x35, 0x00,
        0x12, 0x73, 0x63, 0x65, 0x66, 0x2e, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x2e, 0x70, 0x67, 0x77,
        0x2e, 0x73, 0x65, 0x2e, 0x21, 0x65, 0x70, 0x63, 0x2e, 0x6d, 0x6e, 0x63, 0x30, 0x35, 0x2e,
        0x6d, 0x63, 0x63, 0x32, 0x33, 0x34, 0x2e, 0x33, 0x67, 0x70, 0x70, 0x6e, 0x65, 0x74, 0x77,
        0x6f, 0x72, 0x6b, 0x2e, 0x6f, 0x72, 0x67, 0x2e,
    ];
    let decoded = ScefPdnConnections {
        t: SCEF_PDN_CONN,
        length: 79,
        ins: 0,
        apn: Apn {
            length: 13,
            name: "test.net.com".to_string(),
            ..Apn::default()
        },
        default_ebi: Ebi {
            value: 5,
            ..Ebi::default()
        },
        scef_id: NodeIdentifier {
            t: NODE_ID,
            length: 53,
            ins: 0,
            node_name: "scef.nodes.pgw.se.".to_string(),
            node_realm: "epc.mnc05.mcc234.3gppnetwork.org.".to_string(),
        },
    };
    let i = ScefPdnConnections::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn scef_pdn_connection_ie_marshal_test() {
    let encoded: [u8; 83] = [
        0xc3, 0x00, 0x4f, 0x00, 0x47, 0x00, 0x0d, 0x00, 0x04, 0x74, 0x65, 0x73, 0x74, 0x03, 0x6e,
        0x65, 0x74, 0x03, 0x63, 0x6f, 0x6d, 0x49, 0x00, 0x01, 0x00, 0x05, 0xb0, 0x00, 0x35, 0x00,
        0x12, 0x73, 0x63, 0x65, 0x66, 0x2e, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x2e, 0x70, 0x67, 0x77,
        0x2e, 0x73, 0x65, 0x2e, 0x21, 0x65, 0x70, 0x63, 0x2e, 0x6d, 0x6e, 0x63, 0x30, 0x35, 0x2e,
        0x6d, 0x63, 0x63, 0x32, 0x33, 0x34, 0x2e, 0x33, 0x67, 0x70, 0x70, 0x6e, 0x65, 0x74, 0x77,
        0x6f, 0x72, 0x6b, 0x2e, 0x6f, 0x72, 0x67, 0x2e,
    ];
    let decoded = ScefPdnConnections {
        t: SCEF_PDN_CONN,
        length: 79,
        ins: 0,
        apn: Apn {
            length: 13,
            name: "test.net.com".to_string(),
            ..Apn::default()
        },
        default_ebi: Ebi {
            value: 5,
            ..Ebi::default()
        },
        scef_id: NodeIdentifier {
            t: NODE_ID,
            length: 53,
            ins: 0,
            node_name: "scef.nodes.pgw.se.".to_string(),
            node_realm: "epc.mnc05.mcc234.3gppnetwork.org.".to_string(),
        },
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!(" {:#04x},", x));
    assert_eq!(buffer, encoded);
}
