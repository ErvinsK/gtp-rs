// PGW Change Info IE (Grouped IE) - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{errors::GTPV2Error, messages::ies::*};

// PGW Change Info IE Type

pub const PGW_CHNG_INFO: u8 = 214;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PgwChangeInfo {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub pgw_fqdns: Vec<PgwFqdn>,
    pub sgw_pgw_smf_ips: Vec<IpAddress>,
    pub pgw_smf_fqcsid: Vec<Fqcsid>,
    pub groupids: Vec<GroupId>,
}

impl Default for PgwChangeInfo {
    fn default() -> Self {
        PgwChangeInfo {
            t: PGW_CHNG_INFO,
            length: 0,
            ins: 0,
            pgw_fqdns: vec![],
            sgw_pgw_smf_ips: vec![],
            pgw_smf_fqcsid: vec![],
            groupids: vec![],
        }
    }
}

impl From<PgwChangeInfo> for GroupedIe {
    fn from(i: PgwChangeInfo) -> Self {
        GroupedIe {
            t: PGW_CHNG_INFO,
            length: i.length,
            ins: i.ins,
            elements: i.to_vec(),
        }
    }
}

impl From<GroupedIe> for PgwChangeInfo {
    fn from(i: GroupedIe) -> Self {
        let mut data = PgwChangeInfo::default();
        (data.t, data.length, data.ins) = (i.t, i.length, i.ins);
        for j in i.elements.into_iter() {
            match j {
                InformationElement::PgwFqdn(k) => {
                    if k.ins < 2 {
                        data.pgw_fqdns.push(k)
                    }
                }
                InformationElement::IpAddress(k) => {
                    if k.ins < 4 {
                        data.sgw_pgw_smf_ips.push(k)
                    }
                }
                InformationElement::GroupId(k) => {
                    if k.ins < 2 {
                        data.groupids.push(k)
                    }
                }
                InformationElement::Fqcsid(k) => {
                    if k.ins == 0 {
                        data.pgw_smf_fqcsid.push(k)
                    }
                }
                _ => (),
            }
        }
        data
    }
}

impl IEs for PgwChangeInfo {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let g_ie: GroupedIe = self.clone().into();
        g_ie.marshal(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let data: PgwChangeInfo = match GroupedIe::unmarshal(buffer) {
            Ok(i) => PgwChangeInfo::from(i),
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

impl PgwChangeInfo {
    fn to_vec(&self) -> Vec<InformationElement> {
        let mut v: Vec<InformationElement> = vec![];
        self.sgw_pgw_smf_ips
            .iter()
            .for_each(|i| v.push(i.clone().into()));
        self.pgw_fqdns.iter().for_each(|i| v.push(i.clone().into()));
        self.pgw_smf_fqcsid
            .iter()
            .for_each(|i| v.push(i.clone().into()));
        self.groupids.iter().for_each(|i| v.push(i.clone().into()));
        v
    }
}

#[test]
fn pgw_change_info_ie_unmarshal_test() {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 217] = [
        0xd6, 0x00, 0xd5, 0x00, 0x4a, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4a, 0x00, 0x10,
        0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xd7, 0x00, 0x35, 0x00, 0x05, 0x74, 0x6f, 0x70, 0x6f, 0x6e, 0x05, 0x6e, 0x6f,
        0x64, 0x65, 0x73, 0x03, 0x70, 0x67, 0x77, 0x02, 0x73, 0x65, 0x03, 0x65, 0x70, 0x63, 0x05,
        0x6d, 0x6e, 0x63, 0x30, 0x35, 0x06, 0x6d, 0x63, 0x63, 0x32, 0x33, 0x34, 0x0b, 0x33, 0x67,
        0x70, 0x70, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x03, 0x6f, 0x72, 0x67, 0x00, 0xd7,
        0x00, 0x35, 0x01, 0x05, 0x74, 0x6f, 0x70, 0x6f, 0x6e, 0x05, 0x6e, 0x6f, 0x64, 0x65, 0x73,
        0x03, 0x70, 0x67, 0x77, 0x02, 0x73, 0x65, 0x03, 0x65, 0x70, 0x63, 0x05, 0x6d, 0x6e, 0x63,
        0x30, 0x35, 0x06, 0x6d, 0x63, 0x63, 0x32, 0x33, 0x34, 0x0b, 0x33, 0x67, 0x70, 0x70, 0x6e,
        0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x03, 0x6f, 0x72, 0x67, 0x00, 0xd7, 0x00, 0x35, 0x01,
        0x05, 0x74, 0x6f, 0x70, 0x6f, 0x6e, 0x05, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x03, 0x70, 0x67,
        0x77, 0x02, 0x64, 0x65, 0x03, 0x65, 0x70, 0x63, 0x05, 0x6d, 0x6e, 0x63, 0x30, 0x35, 0x06,
        0x6d, 0x63, 0x63, 0x32, 0x33, 0x34, 0x0b, 0x33, 0x67, 0x70, 0x70, 0x6e, 0x65, 0x74, 0x77,
        0x6f, 0x72, 0x6b, 0x03, 0x6f, 0x72, 0x67, 0x00, 0xd8, 0x00, 0x03, 0x00, 0x0a, 0xff, 0x00,
        0xd8, 0x00, 0x03, 0x00, 0x0a, 0xff, 0x00,
    ];
    let decoded = PgwChangeInfo {
        length: 213,
        sgw_pgw_smf_ips: vec![
            IpAddress {
                t: IP_ADDRESS,
                length: 4,
                ins: 0,
                ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            },
            IpAddress {
                t: IP_ADDRESS,
                length: 16,
                ins: 3,
                ip: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
            },
        ],
        pgw_fqdns: vec![
            PgwFqdn {
                length: 53,
                name: "topon.nodes.pgw.se.epc.mnc05.mcc234.3gppnetwork.org.".to_string(),
                ..PgwFqdn::default()
            },
            PgwFqdn {
                ins: 1,
                length: 53,
                name: "topon.nodes.pgw.se.epc.mnc05.mcc234.3gppnetwork.org.".to_string(),
                ..PgwFqdn::default()
            },
            PgwFqdn {
                length: 53,
                ins: 1,
                name: "topon.nodes.pgw.de.epc.mnc05.mcc234.3gppnetwork.org.".to_string(),
                ..PgwFqdn::default()
            },
        ],
        groupids: vec![
            GroupId {
                length: 3,
                groupid: vec![0x0a, 0xff, 0x00],
                ..GroupId::default()
            },
            GroupId {
                length: 3,
                groupid: vec![0x0a, 0xff, 0x00],
                ..GroupId::default()
            },
        ],
        ..PgwChangeInfo::default()
    };
    let i = PgwChangeInfo::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn pgw_change_info_ie_marshal_test() {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    let encoded: [u8; 217] = [
        0xd6, 0x00, 0xd5, 0x00, 0x4a, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4a, 0x00, 0x10,
        0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xd7, 0x00, 0x35, 0x00, 0x05, 0x74, 0x6f, 0x70, 0x6f, 0x6e, 0x05, 0x6e, 0x6f,
        0x64, 0x65, 0x73, 0x03, 0x70, 0x67, 0x77, 0x02, 0x73, 0x65, 0x03, 0x65, 0x70, 0x63, 0x05,
        0x6d, 0x6e, 0x63, 0x30, 0x35, 0x06, 0x6d, 0x63, 0x63, 0x32, 0x33, 0x34, 0x0b, 0x33, 0x67,
        0x70, 0x70, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x03, 0x6f, 0x72, 0x67, 0x00, 0xd7,
        0x00, 0x35, 0x01, 0x05, 0x74, 0x6f, 0x70, 0x6f, 0x6e, 0x05, 0x6e, 0x6f, 0x64, 0x65, 0x73,
        0x03, 0x70, 0x67, 0x77, 0x02, 0x73, 0x65, 0x03, 0x65, 0x70, 0x63, 0x05, 0x6d, 0x6e, 0x63,
        0x30, 0x35, 0x06, 0x6d, 0x63, 0x63, 0x32, 0x33, 0x34, 0x0b, 0x33, 0x67, 0x70, 0x70, 0x6e,
        0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x03, 0x6f, 0x72, 0x67, 0x00, 0xd7, 0x00, 0x35, 0x01,
        0x05, 0x74, 0x6f, 0x70, 0x6f, 0x6e, 0x05, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x03, 0x70, 0x67,
        0x77, 0x02, 0x64, 0x65, 0x03, 0x65, 0x70, 0x63, 0x05, 0x6d, 0x6e, 0x63, 0x30, 0x35, 0x06,
        0x6d, 0x63, 0x63, 0x32, 0x33, 0x34, 0x0b, 0x33, 0x67, 0x70, 0x70, 0x6e, 0x65, 0x74, 0x77,
        0x6f, 0x72, 0x6b, 0x03, 0x6f, 0x72, 0x67, 0x00, 0xd8, 0x00, 0x03, 0x00, 0x0a, 0xff, 0x00,
        0xd8, 0x00, 0x03, 0x00, 0x0a, 0xff, 0x00,
    ];
    let decoded = PgwChangeInfo {
        length: 213,
        sgw_pgw_smf_ips: vec![
            IpAddress {
                t: IP_ADDRESS,
                length: 4,
                ins: 0,
                ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            },
            IpAddress {
                t: IP_ADDRESS,
                length: 16,
                ins: 3,
                ip: IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
            },
        ],
        pgw_fqdns: vec![
            PgwFqdn {
                length: 53,
                name: "topon.nodes.pgw.se.epc.mnc05.mcc234.3gppnetwork.org.".to_string(),
                ..PgwFqdn::default()
            },
            PgwFqdn {
                ins: 1,
                length: 53,
                name: "topon.nodes.pgw.se.epc.mnc05.mcc234.3gppnetwork.org.".to_string(),
                ..PgwFqdn::default()
            },
            PgwFqdn {
                length: 53,
                ins: 1,
                name: "topon.nodes.pgw.de.epc.mnc05.mcc234.3gppnetwork.org.".to_string(),
                ..PgwFqdn::default()
            },
        ],
        groupids: vec![
            GroupId {
                length: 3,
                groupid: vec![0x0a, 0xff, 0x00],
                ..GroupId::default()
            },
            GroupId {
                length: 3,
                groupid: vec![0x0a, 0xff, 0x00],
                ..GroupId::default()
            },
        ],
        ..PgwChangeInfo::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    //buffer.iter().enumerate().for_each( |x| if (x.0+1) % 16 != 0 {print!("{:#04x},", x.1)} else {println!("{:#04x},", x.1)});
    assert_eq!(buffer, encoded);
}
