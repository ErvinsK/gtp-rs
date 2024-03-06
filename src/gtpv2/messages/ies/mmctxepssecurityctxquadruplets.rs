// MM Context EPS Security Context and Quadruplets IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// MM Context EPS Security Context and Quadruplets IE Type

pub const MMCTXEPSSECCTXQ: u8 = 107;

// MM Context EPS Security Context and Quadruplets IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MmContextEpsSecurityContextQuadruplets {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub sec_mode: SecurityMode,
    pub ksi: u8,
    pub nas_integrity: NasIntegrityProtectionValues,
    pub nas_cipher: NasCipherValues,
    pub nas_dl_count: u32,
    pub nas_ul_count: u32,
    pub kasme: [u8; 32],
    pub auth_quadruplets: Option<Vec<AuthQuadruplet>>,
    pub auth_quintuplets: Option<Vec<AuthQuintuplet>>,
    pub drx_params: Option<[u8; 2]>,
    pub next_hop: Option<[u8; 32]>,
    pub ncc: Option<u8>,
    pub subscr_ue_ambr: Option<AmbrMM>,
    pub used_ue_ambr: Option<AmbrMM>,
    pub ue_ntwk_cap: Option<Vec<u8>>,
    pub ms_ntwk_cap: Option<Vec<u8>>,
    pub mei: Option<Vec<u8>>,
    pub access_res: AccessRestrictionMM,
    pub old_eps_sec_ctx: Option<OldEpsSecurityContext>,
    pub vdn_pref_ue_usage: Option<Vec<u8>>, // Voice domain preference and UE's usage setting
    pub ue_radio_cap_for_paging: Option<Vec<u8>>, // UE Radio capabilitiy for Paging Information
    pub ext_access_res: Option<ExtendedAccessRestrictionMM>,
    pub ue_add_security_cap: Option<Vec<u8>>,
    pub ue_nr_security_cap: Option<Vec<u8>>,
    pub apn_rate_controls: Option<Vec<ApnRateControlStatusMM>>,
    pub core_nw_res: Option<Vec<u8>>, // Core Network Restrictions
    pub ue_radio_cap_id: Option<Vec<u8>>, // UE Radio Capability ID
    pub ensct: Option<u8>,            // EPS NAS Security Context Type
}

impl Default for MmContextEpsSecurityContextQuadruplets {
    fn default() -> Self {
        MmContextEpsSecurityContextQuadruplets {
            t: MMCTXEPSSECCTXQ,
            length: 0,
            ins: 0,
            sec_mode: SecurityMode::EpsSecurityContextAndQuadruplets,
            ksi: 0,
            nas_integrity: NasIntegrityProtectionValues::NoIntegrity,
            nas_cipher: NasCipherValues::NoChiper,
            nas_dl_count: 0,
            nas_ul_count: 0,
            kasme: [0; 32],
            auth_quadruplets: None,
            auth_quintuplets: None,
            drx_params: None,
            next_hop: None,
            ncc: None,
            subscr_ue_ambr: None,
            used_ue_ambr: None,
            ue_ntwk_cap: None,
            ms_ntwk_cap: None,
            mei: None,
            access_res: AccessRestrictionMM::default(),
            old_eps_sec_ctx: None,
            vdn_pref_ue_usage: None, // Voice domain preference and UE's usage setting
            ue_radio_cap_for_paging: None, // UE Radio capabilitiy for Paging Information
            ext_access_res: None,
            ue_add_security_cap: None,
            ue_nr_security_cap: None,
            apn_rate_controls: None,
            core_nw_res: None,     // Core Network Restrictions
            ue_radio_cap_id: None, // UE Radio Capability ID
            ensct: None,           // EPS NAS Security Context Type
        }
    }
}

impl From<MmContextEpsSecurityContextQuadruplets> for InformationElement {
    fn from(i: MmContextEpsSecurityContextQuadruplets) -> Self {
        InformationElement::MmContext(i.into())
    }
}

impl IEs for MmContextEpsSecurityContextQuadruplets {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(MMCTXEPSSECCTXQ);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        {
            let mut byte = u8::from(self.sec_mode.clone()) << 5;
            if self.next_hop.is_some() {
                byte |= 0x10
            };
            if self.drx_params.is_some() {
                byte |= 0x08
            };
            byte |= self.ksi & 0x07;
            buffer_ie.push(byte);
        }
        {
            let mut byte = if let Some(i) = self.auth_quintuplets.clone() {
                (i.len() as u8) << 5
            } else {
                0x00
            };
            if let Some(i) = self.auth_quadruplets.clone() {
                byte |= (i.len() as u8) << 2;
            }
            match (self.used_ue_ambr.is_some(), self.old_eps_sec_ctx.is_some()) {
                (true, true) => byte |= 0x03,
                (true, false) => byte |= 0x02,
                (false, true) => byte |= 0x01,
                (false, false) => (),
            }
            buffer_ie.push(byte);
        }
        {
            let byte: u8 = if self.subscr_ue_ambr.is_some() {
                0x80 | (u8::from(self.nas_integrity.clone())) << 4
                    | u8::from(self.nas_cipher.clone())
            } else {
                (u8::from(self.nas_integrity.clone())) << 4 | u8::from(self.nas_cipher.clone())
            };
            buffer_ie.push(byte);
        }
        buffer_ie.extend_from_slice(&self.nas_dl_count.to_be_bytes()[1..]);
        buffer_ie.extend_from_slice(&self.nas_ul_count.to_be_bytes()[1..]);
        buffer_ie.extend_from_slice(&self.kasme);
        if let Some(i) = self.auth_quadruplets.clone() {
            for quadruplet in i {
                quadruplet.marshal(&mut buffer_ie);
            }
        }
        if let Some(i) = self.auth_quintuplets.clone() {
            for quintuplet in i {
                quintuplet.marshal(&mut buffer_ie);
            }
        }
        if let Some(i) = self.drx_params {
            buffer_ie.extend_from_slice(&i);
        }
        match (self.next_hop, self.ncc) {
            (Some(i), Some(j)) => {
                buffer_ie.extend_from_slice(&i);
                buffer_ie.push(j & 0x07);
            }
            (Some(i), None) => {
                buffer_ie.extend_from_slice(&i);
                buffer_ie.push(0);
            }
            _ => (),
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
        if let Some(i) = self.old_eps_sec_ctx.clone() {
            i.marshal(&mut buffer_ie);
        }
        if let Some(i) = self.vdn_pref_ue_usage.clone() {
            buffer_ie.push(i.len() as u8);
            buffer_ie.extend_from_slice(&i);
        } else {
            buffer_ie.push(0);
        }
        if let Some(i) = self.ue_radio_cap_for_paging.clone() {
            buffer_ie.push(i.len() as u8);
            buffer_ie.extend_from_slice(&i);
        } else {
            buffer_ie.push(0);
        }
        if let Some(i) = self.ext_access_res.clone() {
            buffer_ie.push(0x01);
            buffer_ie.push(u8::from(i));
        } else {
            buffer_ie.push(0x00);
        }
        if let Some(i) = self.ue_add_security_cap.clone() {
            buffer_ie.push(i.len() as u8);
            buffer_ie.extend_from_slice(&i);
        } else {
            buffer_ie.push(0);
        }
        if let Some(i) = self.ue_nr_security_cap.clone() {
            buffer_ie.push(i.len() as u8);
            buffer_ie.extend_from_slice(&i);
        } else {
            buffer_ie.push(0);
        }
        if let Some(i) = self.apn_rate_controls.clone() {
            buffer_ie.push(i.len() as u8);
            for status in i {
                status.marshal(&mut buffer_ie);
            }
        } else {
            buffer_ie.push(0);
        }
        if let Some(i) = self.core_nw_res.clone() {
            buffer_ie.push(i.len() as u8);
            buffer_ie.extend_from_slice(&i);
        } else {
            buffer_ie.push(0);
        }
        if let Some(i) = self.ue_radio_cap_id.clone() {
            buffer_ie.push(i.len() as u8);
            buffer_ie.extend_from_slice(&i);
        } else {
            buffer_ie.push(0);
        }
        if let Some(i) = self.ensct {
            buffer_ie.push(i);
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= 42 + MIN_IE_SIZE {
            let mut data = MmContextEpsSecurityContextQuadruplets {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                sec_mode: SecurityMode::from(buffer[4] >> 5),
                ksi: buffer[4] & 0x07,
                ..MmContextEpsSecurityContextQuadruplets::default()
            };
            let drxi = matches!(buffer[4] & 0x08, 0x08);
            let nhi = matches!(buffer[4] & 0x10, 0x10);
            let auth_quint_i = buffer[5] >> 5;
            let auth_quad_i = (buffer[5] & 0x1c) >> 2;
            let uambri = matches!(buffer[5] & 0x02, 0x02);
            let osci = matches!(buffer[5] & 0x01, 0x01);
            let sambri = matches!(buffer[6] >> 7, 0x01);
            data.nas_integrity = NasIntegrityProtectionValues::from(buffer[6] >> 4 & 0x07);
            data.nas_cipher = NasCipherValues::from(buffer[6] & 0x0f);
            data.nas_dl_count = u32::from_be_bytes([0x00, buffer[7], buffer[8], buffer[9]]);
            data.nas_ul_count = u32::from_be_bytes([0x00, buffer[10], buffer[11], buffer[12]]);
            data.kasme.copy_from_slice(&buffer[13..45]);
            let mut cursor: usize = 45;
            match auth_quad_i {
                0 => (),
                i if i <= 5 => {
                    let mut auth_quadruplets = Vec::new();
                    for j in 0..auth_quad_i {
                        if let Ok(ie) = AuthQuadruplet::unmarshal(&buffer[cursor..]) {
                            auth_quadruplets.push(ie);
                            cursor += auth_quadruplets[j as usize].len();
                        } else {
                            return Err(GTPV2Error::IEIncorrect(MMCTXEPSSECCTXQ));
                        }
                    }
                    data.auth_quadruplets = Some(auth_quadruplets);
                }
                _ => return Err(GTPV2Error::IEIncorrect(MMCTXEPSSECCTXQ)),
            }
            match auth_quint_i {
                0 => (),
                i if i <= 5 => {
                    let mut auth_quintuplets = Vec::new();
                    for j in 0..auth_quint_i {
                        if let Ok(ie) = AuthQuintuplet::unmarshal(&buffer[cursor..]) {
                            auth_quintuplets.push(ie);
                            cursor += auth_quintuplets[j as usize].len();
                        } else {
                            return Err(GTPV2Error::IEIncorrect(MMCTXEPSSECCTXQ));
                        }
                    }
                    data.auth_quintuplets = Some(auth_quintuplets);
                }
                _ => return Err(GTPV2Error::IEIncorrect(MMCTXEPSSECCTXQ)),
            }
            if drxi && buffer.len() >= cursor + 2 {
                data.drx_params = Some([buffer[cursor], buffer[cursor + 1]]);
                cursor += 2;
            }
            if nhi && buffer.len() >= cursor + 33 {
                let mut next_hop = [0; 32];
                next_hop.copy_from_slice(&buffer[cursor..cursor + 32]);
                data.next_hop = Some(next_hop);
                cursor += 32;
                data.ncc = Some(buffer[cursor]);
                cursor += 1;
            }
            if sambri {
                if buffer.len() >= cursor + 8 {
                    if let Ok(ie) = AmbrMM::unmarshal(&buffer[cursor..]) {
                        data.subscr_ue_ambr = Some(ie);
                        cursor += 8;
                    } else {
                        return Err(GTPV2Error::IEIncorrect(MMCTXEPSSECCTXQ));
                    }
                } else {
                    return Err(GTPV2Error::IEInvalidLength(MMCTXEPSSECCTXQ));
                }
            }
            if uambri {
                if buffer.len() >= cursor + 8 {
                    if let Ok(ie) = AmbrMM::unmarshal(&buffer[cursor..]) {
                        data.used_ue_ambr = Some(ie);
                        cursor += 8;
                    } else {
                        return Err(GTPV2Error::IEIncorrect(MMCTXEPSSECCTXQ));
                    }
                } else {
                    return Err(GTPV2Error::IEInvalidLength(MMCTXEPSSECCTXQ));
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
                        return Err(GTPV2Error::IEInvalidLength(MMCTXEPSSECCTXQ));
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
                        return Err(GTPV2Error::IEInvalidLength(MMCTXEPSSECCTXQ));
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
                        return Err(GTPV2Error::IEInvalidLength(MMCTXEPSSECCTXQ));
                    }
                } else {
                    cursor += 1;
                }
            }
            data.access_res = AccessRestrictionMM::from(buffer[cursor]);
            cursor += 1;
            if osci {
                if let Ok(ie) = OldEpsSecurityContext::unmarshal(&buffer[cursor..]) {
                    cursor += ie.len();
                    data.old_eps_sec_ctx = Some(ie);
                } else {
                    return Err(GTPV2Error::IEIncorrect(MMCTXEPSSECCTXQ));
                }
            }
            {
                let len = buffer[cursor] as usize;
                cursor += 1;
                if len > 0 {
                    if buffer.len() >= cursor + len {
                        data.vdn_pref_ue_usage = Some(buffer[cursor..cursor + len].to_vec());
                        cursor += len;
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(MMCTXEPSSECCTXQ));
                    }
                }
            }
            {
                let len = buffer[cursor] as usize;
                cursor += 1;
                if len > 0 {
                    if buffer.len() >= cursor + len {
                        data.ue_radio_cap_for_paging = Some(buffer[cursor..cursor + len].to_vec());
                        cursor += len;
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(MMCTXEPSSECCTXQ));
                    }
                }
            }
            {
                let len = buffer[cursor] as usize;
                match len {
                    0x01 => {
                        cursor += 1;
                        if buffer.len() >= cursor {
                            data.ext_access_res =
                                Some(ExtendedAccessRestrictionMM::from(buffer[cursor]));
                            cursor += 1;
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(MMCTXEPSSECCTXQ));
                        }
                    }
                    0x00 => (),
                    _ => return Err(GTPV2Error::IEInvalidLength(MMCTXEPSSECCTXQ)),
                }
            }
            {
                let len = buffer[cursor] as usize;
                cursor += 1;
                if len > 0 {
                    if buffer.len() >= cursor + len {
                        data.ue_add_security_cap = Some(buffer[cursor..cursor + len].to_vec());
                        cursor += len;
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(MMCTXEPSSECCTXQ));
                    }
                }
            }
            {
                let len = buffer[cursor] as usize;
                cursor += 1;
                if len > 0 {
                    if buffer.len() >= cursor + len {
                        data.ue_nr_security_cap = Some(buffer[cursor..cursor + len].to_vec());
                        cursor += len;
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(MMCTXEPSSECCTXQ));
                    }
                }
            }
            {
                let len = buffer[cursor] as usize;
                match len {
                    0 => data.apn_rate_controls = None,
                    _ => {
                        cursor += 1;
                        if buffer.len() >= cursor + len {
                            let mut apn_rate_controls = Vec::new();
                            while let Ok(ie) = ApnRateControlStatusMM::unmarshal(&buffer[cursor..])
                            {
                                apn_rate_controls.push(ie);
                                cursor += apn_rate_controls.last().unwrap().len();
                            }
                            data.apn_rate_controls = Some(apn_rate_controls);
                        } else {
                            return Err(GTPV2Error::IEInvalidLength(MMCTXEPSSECCTXQ));
                        }
                    }
                }
            }
            {
                let len = buffer[cursor] as usize;
                cursor += 1;
                if len > 0 {
                    if buffer.len() >= cursor + len {
                        data.core_nw_res = Some(buffer[cursor..cursor + len].to_vec());
                        cursor += len;
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(MMCTXEPSSECCTXQ));
                    }
                }
            }
            {
                let len = buffer[cursor] as usize;
                cursor += 1;
                if len > 0 {
                    if buffer.len() >= cursor + len {
                        data.ue_radio_cap_id = Some(buffer[cursor..cursor + len].to_vec());
                        cursor += len;
                    } else {
                        return Err(GTPV2Error::IEInvalidLength(MMCTXEPSSECCTXQ));
                    }
                }
            }
            {
                if buffer.len() >= cursor {
                    data.ensct = Some(buffer[cursor] & 0x03);
                }
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(MMCTXEPSSECCTXQ))
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
fn mmctxepssecctxq_ie_marshal_test() {
    let ie_marshalled: [u8; 396] = [
        0x6b, 0x01, 0x88, 0x00, 0x98, 0x27, 0x81, 0x27, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x03, 0x02, 0x07, 0x08, 0x03, 0x03, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04,
        0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x03, 0x02, 0x07,
        0x08, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f, 0x10, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
        0x0e, 0x0f, 0x10, 0x03, 0x03, 0x09, 0x0a, 0x01, 0x02, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
        0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
        0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0x05, 0x00, 0x00, 0x07,
        0xd0, 0x00, 0x00, 0x1f, 0x40, 0x00, 0x00, 0x07, 0xd0, 0x00, 0x00, 0x1f, 0x40, 0x04, 0x01,
        0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x00, 0xad,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x01,
        0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x02, 0x00, 0x22, 0x00,
        0x0c, 0x74, 0x65, 0x73, 0x74, 0x2e, 0x61, 0x70, 0x6e, 0x2e, 0x63, 0x6f, 0x6d, 0x12, 0x34,
        0x56, 0x78, 0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78, 0x01, 0x02, 0x03, 0x04, 0x05,
        0x06, 0x07, 0x08, 0x00, 0x23, 0x00, 0x0d, 0x74, 0x65, 0x73, 0x74, 0x32, 0x2e, 0x61, 0x70,
        0x6e, 0x2e, 0x63, 0x6f, 0x6d, 0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78, 0x12, 0x34,
        0x56, 0x78, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x04, 0x01, 0x02, 0x03, 0x04,
        0x04, 0x01, 0x02, 0x03, 0x04, 0x02,
    ];
    let ie_to_marshal = MmContextEpsSecurityContextQuadruplets {
        t: MMCTXEPSSECCTXQ,
        length: 392,
        ins: 0,
        sec_mode: SecurityMode::EpsSecurityContextAndQuadruplets,
        ksi: 0,
        nas_integrity: NasIntegrityProtectionValues::NoIntegrity,
        nas_cipher: NasCipherValues::Eea1,
        nas_dl_count: 0x002700ff,
        nas_ul_count: 0x00ffffff,
        kasme: [0xff; 32],
        auth_quadruplets: Some(vec![AuthQuadruplet {
            rand: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
            xres: vec![0x02, 0x07, 0x08],
            autn: vec![0x03, 0x09, 0x0a],
            kasme: [
                0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
        }]),
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
        next_hop: Some([0xaa; 32]),
        ncc: Some(0x05),
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
        old_eps_sec_ctx: Some(OldEpsSecurityContext {
            old_ksi: 5,
            old_ncc: Some(5),
            old_kasme: [0xff; 32],
            old_next_hop: Some([0xff; 32]),
        }),
        vdn_pref_ue_usage: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ue_radio_cap_for_paging: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ext_access_res: Some(ExtendedAccessRestrictionMM::from(0x03)),
        ue_add_security_cap: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ue_nr_security_cap: Some(vec![0x01, 0x02, 0x03, 0x04]),
        apn_rate_controls: Some(vec![
            ApnRateControlStatusMM {
                apn: "test.apn.com".to_string(),
                uplink_rate_limit: 0x12345678,
                nbr_of_exception_reports: 0x12345678,
                downlink_rate_limit: 0x12345678,
                apn_rate_control_status_validity: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
            },
            ApnRateControlStatusMM {
                apn: "test2.apn.com".to_string(),
                uplink_rate_limit: 0x12345678,
                nbr_of_exception_reports: 0x12345678,
                downlink_rate_limit: 0x12345678,
                apn_rate_control_status_validity: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
            },
        ]),
        core_nw_res: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ue_radio_cap_id: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ensct: Some(0x02),
    };
    let mut buffer: Vec<u8> = vec![];
    ie_to_marshal.marshal(&mut buffer);
    //buffer.iter().for_each( |x| print!("{:#04x},", x));
    assert_eq!(buffer, ie_marshalled);
}

#[test]
fn mmctxepssecctxq_ie_unmarshal_test() {
    let ie_marshalled: [u8; 396] = [
        0x6b, 0x01, 0x88, 0x00, 0x98, 0x27, 0x81, 0x27, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x03, 0x02, 0x07, 0x08, 0x03, 0x03, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04,
        0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x03, 0x02, 0x07,
        0x08, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f, 0x10, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
        0x0e, 0x0f, 0x10, 0x03, 0x03, 0x09, 0x0a, 0x01, 0x02, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
        0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
        0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0x05, 0x00, 0x00, 0x07,
        0xd0, 0x00, 0x00, 0x1f, 0x40, 0x00, 0x00, 0x07, 0xd0, 0x00, 0x00, 0x1f, 0x40, 0x04, 0x01,
        0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x00, 0xad,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x01,
        0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x04, 0x01, 0x02, 0x03, 0x04, 0x02, 0x00, 0x22, 0x00,
        0x0c, 0x74, 0x65, 0x73, 0x74, 0x2e, 0x61, 0x70, 0x6e, 0x2e, 0x63, 0x6f, 0x6d, 0x12, 0x34,
        0x56, 0x78, 0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78, 0x01, 0x02, 0x03, 0x04, 0x05,
        0x06, 0x07, 0x08, 0x00, 0x23, 0x00, 0x0d, 0x74, 0x65, 0x73, 0x74, 0x32, 0x2e, 0x61, 0x70,
        0x6e, 0x2e, 0x63, 0x6f, 0x6d, 0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78, 0x12, 0x34,
        0x56, 0x78, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x04, 0x01, 0x02, 0x03, 0x04,
        0x04, 0x01, 0x02, 0x03, 0x04, 0x02,
    ];
    let ie_to_marshal = MmContextEpsSecurityContextQuadruplets {
        t: MMCTXEPSSECCTXQ,
        length: 392,
        ins: 0,
        sec_mode: SecurityMode::EpsSecurityContextAndQuadruplets,
        ksi: 0,
        nas_integrity: NasIntegrityProtectionValues::NoIntegrity,
        nas_cipher: NasCipherValues::Eea1,
        nas_dl_count: 0x002700ff,
        nas_ul_count: 0x00ffffff,
        kasme: [0xff; 32],
        auth_quadruplets: Some(vec![AuthQuadruplet {
            rand: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10,
            ],
            xres: vec![0x02, 0x07, 0x08],
            autn: vec![0x03, 0x09, 0x0a],
            kasme: [
                0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
        }]),
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
        next_hop: Some([0xaa; 32]),
        ncc: Some(0x05),
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
        old_eps_sec_ctx: Some(OldEpsSecurityContext {
            old_ksi: 5,
            old_ncc: Some(5),
            old_kasme: [0xff; 32],
            old_next_hop: Some([0xff; 32]),
        }),
        vdn_pref_ue_usage: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ue_radio_cap_for_paging: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ext_access_res: Some(ExtendedAccessRestrictionMM::from(0x03)),
        ue_add_security_cap: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ue_nr_security_cap: Some(vec![0x01, 0x02, 0x03, 0x04]),
        apn_rate_controls: Some(vec![
            ApnRateControlStatusMM {
                apn: "test.apn.com".to_string(),
                uplink_rate_limit: 0x12345678,
                nbr_of_exception_reports: 0x12345678,
                downlink_rate_limit: 0x12345678,
                apn_rate_control_status_validity: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
            },
            ApnRateControlStatusMM {
                apn: "test2.apn.com".to_string(),
                uplink_rate_limit: 0x12345678,
                nbr_of_exception_reports: 0x12345678,
                downlink_rate_limit: 0x12345678,
                apn_rate_control_status_validity: [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
            },
        ]),
        core_nw_res: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ue_radio_cap_id: Some(vec![0x01, 0x02, 0x03, 0x04]),
        ensct: Some(0x02),
    };
    assert_eq!(
        MmContextEpsSecurityContextQuadruplets::unmarshal(&ie_marshalled).unwrap(),
        ie_to_marshal
    );
}
