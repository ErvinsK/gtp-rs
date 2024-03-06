// MM Context GSM Key, Used Cipher and Quintuplets IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)
use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// MM Context GSM Key, Used Cipher and Quintuplets IE Type

pub const MMCTXGSMKCQ: u8 = 105;

// MM Context GSM Key, Used Cipher and Quintuplets IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MmContextGsmKeyCipherQuintuplets {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub sec_mode: SecurityMode,
    pub cksn: u8,
    pub used_cipher: CipherValues,
    pub kc: u64,
    pub auth_quintuplets: Option<Vec<AuthQuintuplet>>,
    pub drx_params: Option<[u8; 2]>,
    pub subscr_ue_ambr: Option<AmbrMM>,
    pub used_ue_ambr: Option<AmbrMM>,
    pub ue_ntwk_cap: Option<Vec<u8>>,
    pub ms_ntwk_cap: Option<Vec<u8>>,
    pub mei: Option<Vec<u8>>,
    pub access_res: AccessRestrictionMM,
    pub vdn_pref_ue_usage: Option<Vec<u8>>, // Voice domain preference and UE's usage setting
    pub higher_than_16_mbps: bool,
}

impl Default for MmContextGsmKeyCipherQuintuplets {
    fn default() -> Self {
        MmContextGsmKeyCipherQuintuplets {
            t: MMCTXGSMKCQ,
            length: 0,
            ins: 0,
            sec_mode: SecurityMode::GsmKeyUsedCipherAndQuintuplets,
            cksn: 0,
            used_cipher: CipherValues::default(),
            kc: 0,
            auth_quintuplets: None,
            drx_params: None,
            subscr_ue_ambr: None,
            used_ue_ambr: None,
            ue_ntwk_cap: None,
            ms_ntwk_cap: None,
            mei: None,
            access_res: AccessRestrictionMM::default(),
            vdn_pref_ue_usage: None,
            higher_than_16_mbps: false,
        }
    }
}

impl From<MmContextGsmKeyCipherQuintuplets> for InformationElement {
    fn from(i: MmContextGsmKeyCipherQuintuplets) -> Self {
        InformationElement::MmContext(i.into())
    }
}

impl IEs for MmContextGsmKeyCipherQuintuplets {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(MMCTXGSMKCQ);
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
            let mut byte = if let Some(i) = self.auth_quintuplets.clone() {
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
        if let Some(i) = self.auth_quintuplets.clone() {
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
        if self.higher_than_16_mbps {
            buffer_ie.push(0x01);
            buffer_ie.push(0x01);
        } else {
            buffer_ie.push(0x00);
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 16 + MIN_IE_SIZE {
            let mut data = MmContextGsmKeyCipherQuintuplets {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                sec_mode: SecurityMode::from(buffer[4] >> 5),
                cksn: buffer[4] & 0x07,
                ..MmContextGsmKeyCipherQuintuplets::default()
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
                    let mut auth_quintuplets = Vec::new();
                    for j in 0..authi {
                        if let Ok(ie) = AuthQuintuplet::unmarshal(&buffer[cursor..]) {
                            auth_quintuplets.push(ie);
                            cursor += auth_quintuplets[j as usize].len();
                        } else {
                            return Err(GTPV2Error::IEIncorrect(MMCTXGSMKCQ));
                        }
                    }
                    data.auth_quintuplets = Some(auth_quintuplets);
                }
                _ => return Err(GTPV2Error::IEIncorrect(MMCTXGSMKCQ)),
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
                        return Err(GTPV2Error::IEIncorrect(MMCTXGSMKCQ));
                    }
                } else {
                    return Err(GTPV2Error::IEInvalidLength(MMCTXGSMKCQ));
                }
            }
            if uambri {
                if buffer.len() >= cursor + 8 {
                    if let Ok(ie) = AmbrMM::unmarshal(&buffer[cursor..]) {
                        data.used_ue_ambr = Some(ie);
                        cursor += 8;
                    } else {
                        return Err(GTPV2Error::IEIncorrect(MMCTXGSMKCQ));
                    }
                } else {
                    return Err(GTPV2Error::IEInvalidLength(MMCTXGSMKCQ));
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
                        return Err(GTPV2Error::IEInvalidLength(MMCTXGSMKCQ));
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
                        return Err(GTPV2Error::IEInvalidLength(MMCTXGSMKCQ));
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
                        return Err(GTPV2Error::IEInvalidLength(MMCTXGSMKCQ));
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
                        cursor += len;
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(MMCTXGSMKCQ));
                    }
                }
            }
            {
                let len = buffer[cursor] as usize;
                cursor += 1;
                if buffer.len() >= cursor + len {
                    data.higher_than_16_mbps = matches!(buffer[cursor], 0x01);
                }
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(MMCTXGSMKCQ))
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
fn mmctxgsmkcq_ie_marshal_test() {
    let ie_marshalled: [u8; 112] = [
        0x69, 0x00, 0x6c, 0x00, 0x48, 0x23, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x03, 0x02, 0x07, 0x08, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
        0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
        0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x03, 0x03, 0x09, 0x0a, 0x01, 0x02, 0x00, 0x00,
        0x07, 0xd0, 0x00, 0x00, 0x1f, 0x40, 0x00, 0x00, 0x07, 0xd0, 0x00, 0x00, 0x1f, 0x40, 0x04,
        0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x00,
        0x04, 0x01, 0x02, 0x03, 0x04, 0x01, 0x01,
    ];
    let ie_to_marshal = MmContextGsmKeyCipherQuintuplets {
        t: MMCTXGSMKCQ,
        length: 108,
        ins: 0,
        sec_mode: SecurityMode::GsmKeyUsedCipherAndQuintuplets,
        cksn: 0,
        used_cipher: CipherValues::NoCipher,
        kc: 0xffffffffffffffff,
        auth_quintuplets: Some(vec![AuthQuintuplet {
            rand: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
            xres: vec![0x02, 0x07, 0x08],
            ck: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
            ik: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
            autn: vec![0x03, 0x09, 0x0a],
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
        higher_than_16_mbps: true,
    };
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn mmctxgsmkcq_ie_unmarshal_test() {
    let ie_marshalled: [u8; 112] = [
        0x69, 0x00, 0x6c, 0x00, 0x48, 0x23, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x03, 0x02, 0x07, 0x08, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
        0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
        0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x03, 0x03, 0x09, 0x0a, 0x01, 0x02, 0x00, 0x00,
        0x07, 0xd0, 0x00, 0x00, 0x1f, 0x40, 0x00, 0x00, 0x07, 0xd0, 0x00, 0x00, 0x1f, 0x40, 0x04,
        0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x00,
        0x04, 0x01, 0x02, 0x03, 0x04, 0x01, 0x01,
    ];
    let ie_to_marshal = MmContextGsmKeyCipherQuintuplets {
        t: MMCTXGSMKCQ,
        length: 108,
        ins: 0,
        sec_mode: SecurityMode::GsmKeyUsedCipherAndQuintuplets,
        cksn: 0,
        used_cipher: CipherValues::NoCipher,
        kc: 0xffffffffffffffff,
        auth_quintuplets: Some(vec![AuthQuintuplet {
            rand: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
            xres: vec![0x02, 0x07, 0x08],
            ck: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
            ik: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
            autn: vec![0x03, 0x09, 0x0a],
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
        higher_than_16_mbps: true,
    };
    assert_eq!(
        MmContextGsmKeyCipherQuintuplets::unmarshal(&ie_marshalled).unwrap(),
        ie_to_marshal
    );
}
