// Presence Reporting Area Action IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Presence Reporting Area Action IE Type

pub const PRAA: u8 = 177;

// Presence Reporting Area Action IE implementation
//                      Action                                    Value (Decimal)
// Start Reporting changes of UE presence in the PRA                    1
//  Stop Reporting changes of UE presence in the PRA                    2
//Modify Presence Reporting Area elements composing the PRA             3
//                      <spare>                                       0, 4-7

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresenceReportingAreaAction {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub inapra: bool, // Inactive PRA
    pub action: u8,
    pub prai: u32, // 3-octets
    pub tai: Vec<Tai>,
    pub rai: Vec<Rai>,
    pub macro_enb: Vec<MacroEnbId>,
    pub home_enb: Vec<MacroEnbId>,
    pub ecgi: Vec<Ecgi>,
    pub sai: Vec<Sai>,
    pub cgi: Vec<Cgi>,
    pub ext_macro_enb: Vec<ExtMacroEnbId>,
}

impl Default for PresenceReportingAreaAction {
    fn default() -> Self {
        PresenceReportingAreaAction {
            t: PRAA,
            length: 0,
            ins: 0,
            inapra: false,
            action: 0,
            prai: 0,
            tai: vec![],
            rai: vec![],
            macro_enb: vec![],
            home_enb: vec![],
            ecgi: vec![],
            sai: vec![],
            cgi: vec![],
            ext_macro_enb: vec![],
        }
    }
}

impl From<PresenceReportingAreaAction> for InformationElement {
    fn from(i: PresenceReportingAreaAction) -> Self {
        InformationElement::PresenceReportingAreaAction(i)
    }
}

impl IEs for PresenceReportingAreaAction {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(PRAA);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        match self.inapra {
            true => buffer_ie.push(0x08 | self.action),
            false => buffer_ie.push(self.action),
        }
        buffer_ie.extend_from_slice(&self.prai.to_be_bytes()[1..]);
        let (mut tai, mut rai) = (vec![], vec![]);
        {
            let (mut a, mut b) = (self.tai.len(), self.rai.len());
            if a > 15 {
                a = 15;
            }
            if b > 15 {
                b = 15;
            }
            buffer_ie.push((a as u8) << 4 | b as u8);
            for i in self.tai[..a].iter() {
                i.marshal(&mut tai);
            }
            for i in self.rai[..b].iter() {
                i.marshal(&mut rai);
            }
        }
        let mut macro_enb = vec![];
        {
            let cursor = match self.macro_enb.len() {
                i if i <= 63 => i,
                _ => 63,
            };
            buffer_ie.push(cursor as u8);
            for i in self.macro_enb[..cursor].iter() {
                i.marshal(&mut macro_enb);
            }
        }
        let mut home_enb = vec![];
        {
            let cursor = match self.home_enb.len() {
                i if i <= 63 => i,
                _ => 63,
            };
            buffer_ie.push(cursor as u8);
            for i in self.home_enb[..cursor].iter() {
                i.marshal(&mut home_enb);
            }
        }
        let mut ecgi = vec![];
        {
            let cursor = match self.ecgi.len() {
                i if i <= 63 => i,
                _ => 63,
            };
            buffer_ie.push(cursor as u8);
            for i in self.ecgi[..cursor].iter() {
                i.marshal(&mut ecgi);
            }
        }
        let mut sai = vec![];
        {
            let cursor = match self.sai.len() {
                i if i <= 63 => i,
                _ => 63,
            };
            buffer_ie.push(cursor as u8);
            for i in self.sai[..cursor].iter() {
                i.marshal(&mut sai);
            }
        }
        let mut cgi = vec![];
        {
            let cursor = match self.cgi.len() {
                i if i <= 63 => i,
                _ => 63,
            };
            buffer_ie.push(cursor as u8);
            for i in self.cgi[..cursor].iter() {
                i.marshal(&mut cgi);
            }
        }
        buffer_ie.append(&mut tai);
        buffer_ie.append(&mut macro_enb);
        buffer_ie.append(&mut home_enb);
        buffer_ie.append(&mut ecgi);
        buffer_ie.append(&mut rai);
        buffer_ie.append(&mut sai);
        buffer_ie.append(&mut cgi);
        let mut ext_macro_enb = vec![];
        {
            let cursor = match self.ext_macro_enb.len() {
                i if i <= 63 => i,
                _ => 63,
            };
            buffer_ie.push(cursor as u8);
            for i in self.ext_macro_enb[..cursor].iter() {
                i.marshal(&mut ext_macro_enb);
            }
        }
        buffer_ie.append(&mut ext_macro_enb);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = PresenceReportingAreaAction {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..PresenceReportingAreaAction::default()
            };
            if !check_tliv_ie_buffer(data.length, buffer) {
                return Err(GTPV2Error::IEInvalidLength(PRAA));
            }
            match (buffer[4] >> 3) & 0x01 {
                0 => data.inapra = false,
                _ => data.inapra = true,
            }
            data.action = buffer[4] & 0x07;
            data.prai = u32::from_be_bytes([0x00, buffer[5], buffer[6], buffer[7]]);
            let (nbr_tai, nbr_rai, nbr_macro_enb, nbr_home_enb, nbr_ecgi, nbr_sai, nbr_cgi) = (
                buffer[8] >> 4,
                buffer[8] & 0x0f,
                buffer[9] & 0x3f,
                buffer[10] & 0x3f,
                buffer[11] & 0x3f,
                buffer[12] & 0x3f,
                buffer[13] & 0x3f,
            );
            let mut cursor: usize = 14;
            if cursor > buffer.len() {
                return Err(GTPV2Error::IEInvalidLength(PRAA));
            }
            match nbr_tai {
                0 => (),
                _ => {
                    for _ in 0..nbr_tai {
                        if cursor > buffer.len() {
                            return Err(GTPV2Error::IEInvalidLength(PRAA));
                        }
                        match Tai::unmarshal(&buffer[cursor..]) {
                            Ok(j) => {
                                data.tai.push(j);
                                cursor += 5;
                            }
                            Err(_) => return Err(GTPV2Error::IEInvalidLength(PRAA)),
                        }
                    }
                }
            }
            match nbr_macro_enb {
                0 => (),
                _ => {
                    for _ in 0..nbr_macro_enb {
                        if cursor > buffer.len() {
                            return Err(GTPV2Error::IEInvalidLength(PRAA));
                        }
                        match MacroEnbId::unmarshal(&buffer[cursor..]) {
                            Ok(j) => {
                                data.macro_enb.push(j);
                                cursor += 6;
                            }
                            Err(_) => return Err(GTPV2Error::IEInvalidLength(PRAA)),
                        }
                    }
                }
            }
            match nbr_home_enb {
                0 => (),
                _ => {
                    for _ in 0..nbr_home_enb {
                        if cursor > buffer.len() {
                            return Err(GTPV2Error::IEInvalidLength(PRAA));
                        }
                        match MacroEnbId::unmarshal(&buffer[cursor..]) {
                            Ok(j) => {
                                data.home_enb.push(j);
                                cursor += 6;
                            }
                            Err(_) => return Err(GTPV2Error::IEInvalidLength(PRAA)),
                        }
                    }
                }
            }
            match nbr_ecgi {
                0 => (),
                _ => {
                    for _ in 0..nbr_ecgi {
                        if cursor > buffer.len() {
                            return Err(GTPV2Error::IEInvalidLength(PRAA));
                        }
                        match Ecgi::unmarshal(&buffer[cursor..]) {
                            Ok(j) => {
                                data.ecgi.push(j);
                                cursor += 7;
                            }
                            Err(_) => return Err(GTPV2Error::IEInvalidLength(PRAA)),
                        }
                    }
                }
            }
            match nbr_rai {
                0 => (),
                _ => {
                    for _ in 0..nbr_rai {
                        if cursor > buffer.len() {
                            return Err(GTPV2Error::IEInvalidLength(PRAA));
                        }
                        match Rai::unmarshal(&buffer[cursor..]) {
                            Ok(j) => {
                                data.rai.push(j);
                                cursor += 7;
                            }
                            Err(_) => return Err(GTPV2Error::IEInvalidLength(PRAA)),
                        }
                    }
                }
            }
            match nbr_sai {
                0 => (),
                _ => {
                    for _ in 0..nbr_sai {
                        if cursor > buffer.len() {
                            return Err(GTPV2Error::IEInvalidLength(PRAA));
                        }
                        match Sai::unmarshal(&buffer[cursor..]) {
                            Ok(j) => {
                                data.sai.push(j);
                                cursor += 7;
                            }
                            Err(_) => return Err(GTPV2Error::IEInvalidLength(PRAA)),
                        }
                    }
                }
            }
            match nbr_cgi {
                0 => (),
                _ => {
                    for _ in 0..nbr_cgi {
                        if cursor > buffer.len() {
                            return Err(GTPV2Error::IEInvalidLength(PRAA));
                        }
                        match Cgi::unmarshal(&buffer[cursor..]) {
                            Ok(j) => {
                                data.cgi.push(j);
                                cursor += 7;
                            }
                            Err(_) => return Err(GTPV2Error::IEInvalidLength(PRAA)),
                        }
                    }
                }
            }
            if cursor <= buffer.len() {
                let nbr_ext_macro_enb = buffer[cursor];
                cursor += 1;
                match nbr_ext_macro_enb {
                    0 => (),
                    _ => {
                        for _ in 0..nbr_ext_macro_enb {
                            if cursor > buffer.len() {
                                return Err(GTPV2Error::IEInvalidLength(PRAA));
                            }
                            match ExtMacroEnbId::unmarshal(&buffer[cursor..]) {
                                Ok(j) => {
                                    data.ext_macro_enb.push(j);
                                    cursor += 6;
                                }
                                Err(_) => return Err(GTPV2Error::IEInvalidLength(PRAA)),
                            }
                        }
                    }
                }
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(PRAA))
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
fn praa_ie_marshal_test() {
    let encoded: [u8; 66] = [
        0xb1, 0x00, 0x3e, 0x00, 0x01, 0xff, 0xff, 0xff, 0x11, 0x01, 0x01, 0x01, 0x01, 0x01, 0x62,
        0xf2, 0x10, 0x0b, 0xd9, // Tai
        0x62, 0xf2, 0x10, 0x0f, 0xff, 0xff, // Macro eNb
        0x62, 0xf2, 0x10, 0x0f, 0xff, 0xff, // Home eNb
        0x62, 0xf2, 0x10, 0x01, 0xba, 0x40, 0x02, // Ecgi
        0x62, 0xf2, 0x10, 0xff, 0xff, 0xaa, 0xff, // Rai
        0x62, 0xf2, 0x10, 0xff, 0xff, 0xdd, 0xdd, // Sai
        0x62, 0xf2, 0x10, 0xff, 0xff, 0xaa, 0xaa, // Cgi
        0x01, 0x62, 0xf2, 0x10, 0x0f, 0xff, 0xff, // ExtMacroEnbId
    ];
    let decoded = PresenceReportingAreaAction {
        t: PRAA,
        length: 62,
        ins: 0,
        inapra: false,
        action: 1,
        prai: 0xffffff,
        tai: vec![Tai {
            mcc: 262,
            mnc: 1,
            mnc_is_three_digits: false,
            tac: 0x0bd9,
        }],
        rai: vec![Rai {
            mcc: 262,
            mnc: 1,
            mnc_is_three_digits: false,
            lac: 0xffff,
            rac: 0xaa,
        }],
        macro_enb: vec![MacroEnbId {
            mcc: 262,
            mnc: 1,
            mnc_is_three_digits: false,
            macro_id: 0x0fffff,
        }],
        home_enb: vec![MacroEnbId {
            mcc: 262,
            mnc: 1,
            mnc_is_three_digits: false,
            macro_id: 0x0fffff,
        }],
        ecgi: vec![Ecgi {
            mcc: 262,
            mnc: 1,
            mnc_is_three_digits: false,
            eci: 28983298,
        }],
        sai: vec![Sai {
            mcc: 262,
            mnc: 1,
            mnc_is_three_digits: false,
            lac: 0xffff,
            sac: 0xdddd,
        }],
        cgi: vec![Cgi {
            mcc: 262,
            mnc: 1,
            mnc_is_three_digits: false,
            lac: 0xffff,
            ci: 0xaaaa,
        }],
        ext_macro_enb: vec![ExtMacroEnbId {
            mcc: 262,
            mnc: 1,
            mnc_is_three_digits: false,
            smenb: false,
            ext_macro_id: 0x0fffff,
        }],
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

//Ecgi(Ecgi{ mcc: 262, mnc:1, eci:28983298}

#[test]
fn praa_ie_unmarshal_test() {
    let encoded: [u8; 66] = [
        0xb1, 0x00, 0x3e, 0x00, 0x01, 0xff, 0xff, 0xff, 0x11, 0x01, 0x01, 0x01, 0x01, 0x01, 0x62,
        0xf2, 0x10, 0x0b, 0xd9, // Tai
        0x62, 0xf2, 0x10, 0xff, 0xff, 0xff, // Macro eNb
        0x62, 0xf2, 0x10, 0xff, 0xff, 0xff, // Home eNb
        0x62, 0xf2, 0x10, 0x01, 0xba, 0x40, 0x02, // Ecgi
        0x62, 0xf2, 0x10, 0xff, 0xff, 0xaa, 0xff, // Rai
        0x62, 0xf2, 0x10, 0xff, 0xff, 0xdd, 0xdd, // Sai
        0x62, 0xf2, 0x10, 0xff, 0xff, 0xaa, 0xaa, // Cgi
        0x01, 0x62, 0xf2, 0x10, 0x0f, 0xff, 0xff,
    ]; // ExtMacroEnbId
    let decoded = PresenceReportingAreaAction {
        t: PRAA,
        length: 62,
        ins: 0,
        inapra: false,
        action: 1,
        prai: 0xffffff,
        tai: vec![Tai {
            mcc: 262,
            mnc: 1,
            mnc_is_three_digits: false,
            tac: 0x0bd9,
        }],
        rai: vec![Rai {
            mcc: 262,
            mnc: 1,
            mnc_is_three_digits: false,
            lac: 0xffff,
            rac: 0xaa,
        }],
        macro_enb: vec![MacroEnbId {
            mcc: 262,
            mnc: 1,
            mnc_is_three_digits: false,
            macro_id: 0xffffff,
        }],
        home_enb: vec![MacroEnbId {
            mcc: 262,
            mnc: 1,
            mnc_is_three_digits: false,
            macro_id: 0xffffff,
        }],
        ecgi: vec![Ecgi {
            mcc: 262,
            mnc: 1,
            mnc_is_three_digits: false,
            eci: 28983298,
        }],
        sai: vec![Sai {
            mcc: 262,
            mnc: 1,
            mnc_is_three_digits: false,
            lac: 0xffff,
            sac: 0xdddd,
        }],
        cgi: vec![Cgi {
            mcc: 262,
            mnc: 1,
            mnc_is_three_digits: false,
            lac: 0xffff,
            ci: 0xaaaa,
        }],
        ext_macro_enb: vec![ExtMacroEnbId {
            mcc: 262,
            mnc: 1,
            mnc_is_three_digits: false,
            smenb: false,
            ext_macro_id: 0x0fffff,
        }],
    };
    assert_eq!(
        PresenceReportingAreaAction::unmarshal(&encoded).unwrap(),
        decoded
    );
}
