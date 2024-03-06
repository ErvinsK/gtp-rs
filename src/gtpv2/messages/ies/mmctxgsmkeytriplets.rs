// MM Context GSM Key and Triplets IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// MM Context GSM Key and Triplets IE Type

pub const MMCTXGSMKT: u8 = 103;

// MM Context GSM Key and Triplets IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MmContextGsmKeyTriplets {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub sec_mode: SecurityMode,
    pub cksn: u8,
    pub used_cipher: CipherValues,
    pub kc: u64,
    pub auth_triplets: Option<Vec<AuthTriplet>>,
    pub drx_params: Option<[u8; 2]>,
    pub subscr_ue_ambr: Option<AmbrMM>,
    pub used_ue_ambr: Option<AmbrMM>,
    pub ue_ntwk_cap: Option<Vec<u8>>,
    pub ms_ntwk_cap: Option<Vec<u8>>,
    pub mei: Option<Vec<u8>>,
    pub access_res: AccessRestrictionMM,
    pub vdn_pref_ue_usage: Option<Vec<u8>>, // Voice domain preference and UE's usage setting
}

impl Default for MmContextGsmKeyTriplets {
    fn default() -> Self {
        MmContextGsmKeyTriplets {
            t: MMCTXGSMKT,
            length: 0,
            ins: 0,
            sec_mode: SecurityMode::GsmKeyAndTriplets,
            cksn: 0,
            used_cipher: CipherValues::default(),
            kc: 0,
            auth_triplets: None,
            drx_params: None,
            subscr_ue_ambr: None,
            used_ue_ambr: None,
            ue_ntwk_cap: None,
            ms_ntwk_cap: None,
            mei: None,
            access_res: AccessRestrictionMM::default(),
            vdn_pref_ue_usage: None,
        }
    }
}

impl From<MmContextGsmKeyTriplets> for InformationElement {
    fn from(i: MmContextGsmKeyTriplets) -> Self {
        InformationElement::MmContext(i.into())
    }
}

impl IEs for MmContextGsmKeyTriplets {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(MMCTXGSMKT);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        {
            let byte = match self.drx_params {
                Some(_) => u8::from(self.sec_mode.clone()) << 5 | 0x08 | self.cksn & 0x07,
                None => u8::from(self.sec_mode.clone()) << 5 | self.cksn & 0x07,
            };
            buffer_ie.push(byte);
        }
        {
            let mut byte = if let Some(i) = self.auth_triplets.clone() {
                (i.len() as u8) << 5
            } else {
                0x00
            };
            match (self.used_ue_ambr.is_some(), self.subscr_ue_ambr.is_some()) {
                (true, true) => byte |= 0x03,
                (true, false) => byte |= 0x02,
                (false, true) => byte |= 0x01,
                (false, false) => (),
            }
            buffer_ie.push(byte);
        }
        buffer_ie.push(u8::from(self.used_cipher.clone()));
        buffer_ie.extend_from_slice(&self.kc.to_be_bytes());
        if let Some(i) = self.auth_triplets.clone() {
            for triplet in i {
                triplet.marshal(&mut buffer_ie);
            }
        }
        if let Some(i) = self.drx_params {
            buffer_ie.extend_from_slice(&i);
        }
        if let Some(i) = self.subscr_ue_ambr.clone() {
            i.marshal(&mut buffer_ie);
        }
        if let Some(i) = self.used_ue_ambr.clone() {
            i.marshal(&mut buffer_ie);
        }
        if let Some(i) = self.ue_ntwk_cap.clone() {
            buffer_ie.push(i.len() as u8);
            buffer_ie.extend_from_slice(&i);
        } else {
            buffer_ie.push(0);
        }
        if let Some(i) = self.ms_ntwk_cap.clone() {
            buffer_ie.push(i.len() as u8);
            buffer_ie.extend_from_slice(&i);
        } else {
            buffer_ie.push(0);
        }
        if let Some(i) = self.mei.clone() {
            buffer_ie.push(i.len() as u8);
            buffer_ie.extend_from_slice(&i);
        } else {
            buffer_ie.push(0);
        }
        buffer_ie.push(u8::from(self.access_res.clone()));
        if let Some(i) = self.vdn_pref_ue_usage.clone() {
            buffer_ie.push(i.len() as u8);
            buffer_ie.extend_from_slice(&i);
        } else {
            buffer_ie.push(0);
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 16 + MIN_IE_SIZE {
            let mut data = MmContextGsmKeyTriplets {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                sec_mode: SecurityMode::from(buffer[4] >> 5),
                cksn: buffer[4] & 0x07,
                ..MmContextGsmKeyTriplets::default()
            };
            let drxi = matches!(buffer[4] & 0x08, 0x08);
            let authi = buffer[5] >> 5;
            let uambri = matches!(buffer[5] & 0x02, 0x02);
            let sambri = matches!(buffer[5] & 0x01, 0x01);
            data.used_cipher = CipherValues::from(buffer[6]);
            data.kc = u64::from_slice(&buffer[7..15]);
            let mut cursor: usize = 15;
            match authi {
                0 => (),
                i if i <= 5 => {
                    if buffer.len() >= cursor + (authi as usize * AuthTriplet::default().len()) {
                        let mut auth_triplets = Vec::new();
                        for _ in 0..authi {
                            if let Ok(ie) = AuthTriplet::unmarshal(&buffer[cursor..]) {
                                auth_triplets.push(ie);
                                cursor += AuthTriplet::default().len();
                            } else {
                                return Err(GTPV2Error::IEIncorrect(MMCTXGSMKT));
                            }
                        }
                        data.auth_triplets = Some(auth_triplets);
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(MMCTXGSMKT));
                    }
                }
                _ => return Err(GTPV2Error::IEIncorrect(MMCTXGSMKT)),
            }
            if drxi && buffer.len() >= cursor + 2 {
                data.drx_params = Some([buffer[cursor], buffer[cursor + 1]]);
                cursor += 2;
            }
            if sambri {
                if buffer.len() >= cursor + 8 {
                    if let Ok(ie) = AmbrMM::unmarshal(&buffer[cursor..]) {
                        data.subscr_ue_ambr = Some(ie);
                        cursor += 8;
                    } else {
                        return Err(GTPV2Error::IEIncorrect(MMCTXGSMKT));
                    }
                } else {
                    return Err(GTPV2Error::IEInvalidLength(MMCTXGSMKT));
                }
            }
            if uambri {
                if buffer.len() >= cursor + 8 {
                    if let Ok(ie) = AmbrMM::unmarshal(&buffer[cursor..]) {
                        data.used_ue_ambr = Some(ie);
                        cursor += 8;
                    } else {
                        return Err(GTPV2Error::IEIncorrect(MMCTXGSMKT));
                    }
                } else {
                    return Err(GTPV2Error::IEInvalidLength(MMCTXGSMKT));
                }
            }
            {
                let len = buffer[cursor] as usize;
                cursor += 1;
                if len > 0 {
                    if buffer.len() >= cursor + len {
                        data.ue_ntwk_cap = Some(buffer[cursor..cursor + len].to_vec());
                        cursor += len;
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(MMCTXGSMKT));
                    }
                } else {
                    cursor += 1;
                }
            }
            {
                let len = buffer[cursor] as usize;
                cursor += 1;
                if len > 0 {
                    if buffer.len() >= cursor + len {
                        data.ms_ntwk_cap = Some(buffer[cursor..cursor + len].to_vec());
                        cursor += len;
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(MMCTXGSMKT));
                    }
                } else {
                    cursor += 1;
                }
            }
            {
                let len = buffer[cursor] as usize;
                cursor += 1;
                if len > 0 {
                    if buffer.len() >= cursor + len {
                        data.mei = Some(buffer[cursor..cursor + len].to_vec());
                        cursor += len;
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(MMCTXGSMKT));
                    }
                } else {
                    cursor += 1;
                }
            }
            data.access_res = AccessRestrictionMM::from(buffer[cursor]);
            cursor += 1;
            {
                let len = buffer[cursor] as usize;
                cursor += 1;
                if len > 0 {
                    if buffer.len() >= cursor + len {
                        data.vdn_pref_ue_usage = Some(buffer[cursor..cursor + len].to_vec());
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(MMCTXGSMKT));
                    }
                }
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(MMCTXGSMKT))
        }
    }

    fn len(&self) -> usize {
        self.length as usize + 4
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
fn mmctxgsmkt_ie_marshal_test() {
    let ie_marshalled: [u8; 82] = [
        0x67, 0x00, 0x4e, 0x00, 0x08, 0x23, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x01, 0x02,
        0x00, 0x00, 0x07, 0xd0, 0x00, 0x00, 0x1f, 0x40, 0x00, 0x00, 0x07, 0xd0, 0x00, 0x00, 0x1f,
        0x40, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03,
        0x04, 0x00, 0x04, 0x01, 0x02, 0x03, 0x04,
    ];
    let ie_to_marshal = MmContextGsmKeyTriplets {
        t: MMCTXGSMKT,
        length: 78,
        ins: 0,
        sec_mode: SecurityMode::GsmKeyAndTriplets,
        cksn: 0,
        used_cipher: CipherValues::NoCipher,
        kc: 0xffffffffffffffff,
        auth_triplets: Some(vec![AuthTriplet {
            rand: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
            sres: [0x11, 0x12, 0x13, 0x14],
            kc: [0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c],
        }]),
        drx_params: Some([0x01, 0x02]),
        subscr_ue_ambr: Some(AmbrMM {
            uplink: 2000,
            downlink: 8000,
        }),
        used_ue_ambr: Some(AmbrMM {
            uplink: 2000,
            downlink: 8000,
        }),
        ue_ntwk_cap: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ms_ntwk_cap: Some(vec![0x01, 0x02, 0x03, 0x04]),
        mei: Some(vec![0x01, 0x02, 0x03, 0x04]),
        access_res: AccessRestrictionMM::from(0x00),
        vdn_pref_ue_usage: Some(vec![0x01, 0x02, 0x03, 0x04]),
    };
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn mmctxgsmkt_ie_unmarshal_test() {
    let ie_marshalled: [u8; 82] = [
        0x67, 0x00, 0x4e, 0x00, 0x08, 0x23, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x01, 0x02,
        0x00, 0x00, 0x07, 0xd0, 0x00, 0x00, 0x1f, 0x40, 0x00, 0x00, 0x07, 0xd0, 0x00, 0x00, 0x1f,
        0x40, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03,
        0x04, 0x00, 0x04, 0x01, 0x02, 0x03, 0x04,
    ];
    let ie_to_marshal = MmContextGsmKeyTriplets {
        t: MMCTXGSMKT,
        length: 78,
        ins: 0,
        sec_mode: SecurityMode::GsmKeyAndTriplets,
        cksn: 0,
        used_cipher: CipherValues::NoCipher,
        kc: 0xffffffffffffffff,
        auth_triplets: Some(vec![AuthTriplet {
            rand: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
            sres: [0x11, 0x12, 0x13, 0x14],
            kc: [0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c],
        }]),
        drx_params: Some([0x01, 0x02]),
        subscr_ue_ambr: Some(AmbrMM {
            uplink: 2000,
            downlink: 8000,
        }),
        used_ue_ambr: Some(AmbrMM {
            uplink: 2000,
            downlink: 8000,
        }),
        ue_ntwk_cap: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ms_ntwk_cap: Some(vec![0x01, 0x02, 0x03, 0x04]),
        mei: Some(vec![0x01, 0x02, 0x03, 0x04]),
        access_res: AccessRestrictionMM::from(0x00),
        vdn_pref_ue_usage: Some(vec![0x01, 0x02, 0x03, 0x04]),
    };
    assert_eq!(
        MmContextGsmKeyTriplets::unmarshal(&ie_marshalled).unwrap(),
        ie_to_marshal
    );
}
