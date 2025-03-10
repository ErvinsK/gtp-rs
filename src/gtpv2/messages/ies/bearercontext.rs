// Bearer Context IE (Grouped IE) - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{errors::GTPV2Error, messages::ies::*};

// Bearer Context IE Type

pub const BEARER_CTX: u8 = 93;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BearerContext {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub ebi: Ebi,
    pub cause: Option<Cause>,
    pub tft: Option<BearerTft>,
    pub fteids: Vec<Fteid>,
    pub bearer_qos: Option<BearerQos>,
    pub charging_id: Option<ChargingId>,
    pub bearer_flags: Option<BearerFlags>,
    pub pco: Option<Pco>,
    pub apco: Option<Apco>,
    pub epco: Option<Epco>,
    pub max_packet_loss: Option<MaxPacketLossRate>,
    pub ran_nas_cause: Option<RanNasCause>,
    pub bss_container: Option<Fcontainer>, // Specific to S3/S10/S16/N26
    pub transaction_id: Option<TransactionIdentifier>, // Specific to S3/S10/S16/N26
}

impl Default for BearerContext {
    fn default() -> Self {
        BearerContext {
            t: BEARER_CTX,
            length: 0,
            ins: 0,
            ebi: Ebi::default(),
            cause: None,
            tft: None,
            fteids: vec![],
            bearer_qos: None,
            charging_id: None,
            bearer_flags: None,
            pco: None,
            apco: None,
            epco: None,
            max_packet_loss: None,
            ran_nas_cause: None,
            bss_container: None,
            transaction_id: None,
        }
    }
}

impl From<BearerContext> for GroupedIe {
    fn from(i: BearerContext) -> Self {
        GroupedIe {
            t: BEARER_CTX,
            length: i.length,
            ins: i.ins,
            elements: i.to_vec(),
        }
    }
}

impl From<GroupedIe> for BearerContext {
    fn from(i: GroupedIe) -> Self {
        let mut bearer = BearerContext::default();
        (bearer.t, bearer.length, bearer.ins) = (i.t, i.length, i.ins);
        let mut mandatory = false;
        for j in i.elements.into_iter() {
            match j {
                InformationElement::Ebi(k) => {
                    if let (0, false) = (k.ins, mandatory) {
                        (bearer.ebi, mandatory) = (k, true)
                    };
                }
                InformationElement::Cause(k) => {
                    if let (0, true) = (k.ins, bearer.cause.is_none()) {
                        bearer.cause = Some(k)
                    };
                }
                InformationElement::BearerTft(k) => {
                    if let (0, true) = (k.ins, bearer.tft.is_none()) {
                        bearer.tft = Some(k)
                    };
                }
                InformationElement::Fteid(k) => bearer.fteids.push(k),
                InformationElement::BearerQos(k) => {
                    if let (0, true) = (k.ins, bearer.bearer_qos.is_none()) {
                        bearer.bearer_qos = Some(k)
                    };
                }
                InformationElement::ChargingId(k) => {
                    if let (0, true) = (k.ins, bearer.charging_id.is_none()) {
                        bearer.charging_id = Some(k)
                    };
                }
                InformationElement::BearerFlags(k) => {
                    if let (0, true) = (k.ins, bearer.bearer_flags.is_none()) {
                        bearer.bearer_flags = Some(k)
                    };
                }
                InformationElement::Pco(k) => {
                    if let (0, true) = (k.ins, bearer.pco.is_none()) {
                        bearer.pco = Some(k)
                    };
                }
                InformationElement::Apco(k) => {
                    if let (0, true) = (k.ins, bearer.apco.is_none()) {
                        bearer.apco = Some(k)
                    };
                }
                InformationElement::Epco(k) => {
                    if let (0, true) = (k.ins, bearer.epco.is_none()) {
                        bearer.epco = Some(k)
                    };
                }
                InformationElement::MaxPacketLossRate(k) => {
                    if let (0, true) = (k.ins, bearer.max_packet_loss.is_none()) {
                        bearer.max_packet_loss = Some(k)
                    };
                }
                InformationElement::RanNasCause(k) => {
                    if let (0, true) = (k.ins, bearer.ran_nas_cause.is_none()) {
                        bearer.ran_nas_cause = Some(k)
                    };
                }
                InformationElement::Fcontainer(k) => {
                    if let (0, true) = (k.ins, bearer.bss_container.is_none()) {
                        bearer.bss_container = Some(k)
                    };
                }
                InformationElement::TransactionIdentifier(k) => {
                    if let (0, true) = (k.ins, bearer.transaction_id.is_none()) {
                        bearer.transaction_id = Some(k)
                    };
                }
                _ => (),
            }
        }
        bearer
    }
}

impl IEs for BearerContext {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let g_ie = GroupedIe::from(self.clone());
        g_ie.marshal(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let data: BearerContext = match GroupedIe::unmarshal(buffer) {
            Ok(i) => BearerContext::from(i),
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

impl BearerContext {
    fn to_vec(&self) -> Vec<InformationElement> {
        let mut v: Vec<InformationElement> = vec![];

        if let Some(i) = self.cause.clone() {
            v.push(i.into())
        }

        v.push(self.ebi.clone().into());

        if let Some(i) = self.pco.clone() {
            v.push(i.into())
        }

        if let Some(i) = self.bearer_qos.clone() {
            v.push(i.into())
        }

        if let Some(i) = self.tft.clone() {
            v.push(i.into())
        }

        self.fteids
            .iter()
            .for_each(|x| v.push(InformationElement::Fteid(x.clone())));

        if let Some(i) = self.charging_id.clone() {
            v.push(i.into())
        }

        if let Some(i) = self.bearer_flags.clone() {
            v.push(i.into())
        }

        if let Some(i) = self.apco.clone() {
            v.push(i.into())
        }

        if let Some(i) = self.ran_nas_cause.clone() {
            v.push(i.into())
        }

        if let Some(i) = self.epco.clone() {
            v.push(i.into())
        }

        if let Some(i) = self.max_packet_loss.clone() {
            v.push(i.into())
        }

        v
    }
}

#[test]
fn bearer_context_ie_unmarshal_test() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 62] = [
        0x5d, 0x00, 0x3a, 0x00, 0x02, 0x00, 0x02, 0x00, 0x10, 0x00, 0x49, /* ...].,.I */
        0x00, 0x01, 0x00, 0x05, 0x57, 0x00, 0x09, 0x02, /* ....W... */
        0x84, 0x06, 0xd1, 0x82, 0x4c, 0xc1, 0xfe, 0x8b, /* ....L... */
        0x2d, 0x50, 0x00, 0x16, 0x00, 0x6c, 0x09, 0x00, /* -P...l.. */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* ........ */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* ........ */
        0x00, 0x00, 0x00, 0x5e, 0x00, 0x04, 0x00, 0x05, 0x43, 0x67, 0xdf,
    ];
    let decoded = BearerContext {
        t: 93,
        length: 58,
        ins: 0,
        ebi: Ebi {
            t: 73,
            length: 1,
            ins: 0,
            value: 5,
        },
        cause: Some(Cause {
            t: CAUSE,
            length: 2,
            ins: 0,
            value: 16,
            pce: false,
            bce: false,
            cs: false,
            offend_ie_type: None,
        }),
        tft: None,
        fteids: vec![Fteid {
            t: 87,
            length: 9,
            ins: 2,
            interface: 4,
            teid: 114393676,
            ipv4: Some(Ipv4Addr::new(193, 254, 139, 45)),
            ipv6: None,
        }],
        bearer_qos: Some(BearerQos {
            t: 80,
            length: 22,
            ins: 0,
            pre_emption_vulnerability: 0,
            priority_level: 11,
            pre_emption_capability: 1,
            qci: 9,
            maxbr_ul: 0,
            maxbr_dl: 0,
            gbr_ul: 0,
            gbr_dl: 0,
        }),
        charging_id: Some(ChargingId {
            t: CHARGINGID,
            length: 4,
            ins: 0,
            charging_id: 0x54367df,
        }),
        ..BearerContext::default()
    };
    let i = BearerContext::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn bearer_context_ie_marshal_test() {
    use std::net::Ipv4Addr;
    let encoded: [u8; 62] = [
        0x5d, 0x00, 0x3a, 0x00, 0x02, 0x00, 0x02, 0x00, 0x10, 0x00, 0x49, /* ...].,.I */
        0x00, 0x01, 0x00, 0x05, 0x57, 0x00, 0x09, 0x02, /* ....W... */
        0x84, 0x06, 0xd1, 0x82, 0x4c, 0xc1, 0xfe, 0x8b, /* ....L... */
        0x2d, 0x50, 0x00, 0x16, 0x00, 0x6c, 0x09, 0x00, /* -P...l.. */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* ........ */
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* ........ */
        0x00, 0x00, 0x00, 0x5e, 0x00, 0x04, 0x00, 0x05, 0x43, 0x67, 0xdf,
    ];
    let decoded = BearerContext {
        t: 93,
        length: 58,
        ins: 0,
        ebi: Ebi {
            t: 73,
            length: 1,
            ins: 0,
            value: 5,
        },
        cause: Some(Cause {
            t: CAUSE,
            length: 2,
            ins: 0,
            value: 16,
            pce: false,
            bce: false,
            cs: false,
            offend_ie_type: None,
        }),
        tft: None,
        fteids: vec![Fteid {
            t: 87,
            length: 9,
            ins: 2,
            interface: 4,
            teid: 114393676,
            ipv4: Some(Ipv4Addr::new(193, 254, 139, 45)),
            ipv6: None,
        }],
        bearer_qos: Some(BearerQos {
            t: 80,
            length: 22,
            ins: 0,
            pre_emption_vulnerability: 0,
            priority_level: 11,
            pre_emption_capability: 1,
            qci: 9,
            maxbr_ul: 0,
            maxbr_dl: 0,
            gbr_ul: 0,
            gbr_dl: 0,
        }),
        charging_id: Some(ChargingId {
            t: CHARGINGID,
            length: 4,
            ins: 0,
            charging_id: 0x54367df,
        }),
        ..BearerContext::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}
