// Indication IE - according to 3GPP TS 29.274 V17.10.0 (2023-12)
use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// Indication IE TL

pub const INDICATION: u8 = 77;
pub const INDICATION_LENGTH: usize = 10;

// Indication IE implementation

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Indication {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub daf: bool,     // Dual Address Bearer Flag
    pub dtf: bool,     // Direct Tunnel Flag
    pub hi: bool,      // Handover Indication
    pub dfi: bool,     // Direct Forwarding Indication
    pub oi: bool,      // Operation Indication
    pub isrsi: bool,   // Idle mode Signalling Reduction Supported Indication
    pub israi: bool,   // Idle mode Signalling Reduction Activation Indication
    pub sgwci: bool,   // SGW Change Indication
    pub sqci: bool,    // Subscribed QoS Change Indication
    pub uimsi: bool,   // Unauthenticated IMSI
    pub cfsi: bool,    // Change F-TEID support Indication
    pub crsi: bool,    // Change Reporting Support Indication
    pub ps: bool,      // Piggybacking Supported
    pub pt: bool,      // S5/S8 Protocol Type
    pub si: bool,      // Scope Indication
    pub msv: bool,     // MS Validated
    pub retloc: bool,  // Retrieve Location Indication Flag
    pub pbic: bool,    // Propagate BBAI Information Change
    pub srni: bool,    // SGW Restoration Needed Indication
    pub s6af: bool,    // Static IPv6 Address Flag
    pub s4af: bool,    // Static IPv4 Address Flag
    pub mbmdt: bool,   // Management Based MDT allowed flag
    pub israu: bool,   // ISR is activated for the UE
    pub ccrsi: bool,   // CSG Change Reporting Support Indication
    pub cprai: bool,   // Change of Presence Reporting Area information Indication
    pub arrl: bool,    // Abnormal Release of Radio Link
    pub ppoff: bool,   // PDN Pause Off Indication
    pub ppon: bool,    // PDN Pause On Indication
    pub ppsi: bool,    // PDN Pause Support Indication
    pub csfbi: bool,   // CSFB Indication
    pub clii: bool,    // Change of Location Information Indication
    pub cpsr: bool,    // CS to PS SRVCC indication
    pub nsi: bool,     // NBIFOM Support Indication
    pub uasi: bool,    // UE Available for Signalling Indication
    pub dtci: bool,    // Delay Tolerant Connection Indication
    pub bdwi: bool,    // Buffered DL Data Waiting Indication
    pub psci: bool,    // Pending Subscription Change Indication
    pub pcri: bool,    // P-CSCF Restoration Indication
    pub aosi: bool,    // Associate OCI with SGW node's Identity
    pub aopi: bool,    // Associate OCI with PGW node's Identity
    pub roaai: bool,   // Release Over Any Access Indication
    pub epcosi: bool,  // Extended PCO Support Indication
    pub cpopci: bool,  // Control Plane Only PDN Connection Indication
    pub pmtsmi: bool,  // Pending MT Short Message Indication
    pub s11tf: bool,   // S11-U Tunnel Flag
    pub pnsi: bool,    // Pending Network Initiated PDN Connection Signalling Indication
    pub unaccsi: bool, // UE Not Authorised Cause Code Support Indication
    pub wpmsi: bool,   // WLCP PDN Connection Modification Support Indication
    pub g5snn26: bool, // 5GS Interworking without N26 indication
    pub reprefi: bool, // Return Preferred Indication
    pub g5siwki: bool, // 5GS Interworking Indication
    pub eevrsi: bool,  // Extended EBI Value Range Support Indication
    pub ltemui: bool,  // LTE-M UE Indication
    pub ltempi: bool,  // LTE-M RAT Type reporting to PGW Indication
    pub enbcrsi: bool, // eNB Change Reporting Support Indication
    pub tspcmi: bool,  // Triggering SGSN initiated PDP Context Creation/Modification Indication
    pub csrmfi: bool,  // Create Session Request Message Forwarded Indication
    pub mtedtn: bool,  // MT-EDT Not Applicable Indication
    pub mtedta: bool,  // MT-EDT Applicable Indication
    pub n5gnmi: bool,  // No 5GS N26 Mobility Indication
    pub g5cnrs: bool,  // 5GC Not Restricted Support Indication
    pub g5cnri: bool,  // 5GC Not Restricted Indication
    pub s5rho: bool,   // 5G-SRVCC Handover Indication
    pub ethpdn: bool,  // Ethernet PDN Support Indication
    pub nspusi: bool,  // Notify Start Pause of charging via User plane Support Indication
    pub pgwrnsi: bool, // PGW Redirection due to mismatch with Network Slice subscribed by UE Support Indication
    pub rppcsi: bool, // Restoration of PDN connections after an PGW-C/SMF Change Support Indication
    pub pgwchi: bool, // PGW CHange Indication
    pub sissme: bool, // Same IWK-SCEF Selected for Monitoring Event Indication
    pub nsenbi: bool, // Notify Source eNodeB Indication
    pub idfupf: bool, // Indirect Data Forwarding with UPF Indication
    pub emci: bool,   // Emergency PDU Session Indication
    pub ltemsai: bool, // LTE-M Satellite Access Indication
    pub srtpi: bool,  // Satellite RAT Type reporting to PGW Indication
    pub upipsi: bool, // User Plane Integrity Protection Support Indication
}

impl Default for Indication {
    fn default() -> Self {
        Indication {
            t: INDICATION,
            length: INDICATION_LENGTH as u16,
            ins: 0,
            daf: false,     // Dual Address Bearer Flag
            dtf: false,     // Direct Tunnel Flag
            hi: false,      // Handover Indication
            dfi: false,     // Direct Forwarding Indication
            oi: false,      // Operation Indication
            isrsi: false,   // Idle mode Signalling Reduction Supported Indication
            israi: false,   // Idle mode Signalling Reduction Activation Indication
            sgwci: false,   // SGW Change Indication
            sqci: false,    // Subscribed QoS Change Indication
            uimsi: false,   // Unauthenticated IMSI
            cfsi: false,    // Change F-TEID support Indication
            crsi: false,    // Change Reporting Support Indication
            ps: false,      // Piggybacking Supported
            pt: false,      // S5/S8 Protocol Type
            si: false,      // Scope Indication
            msv: false,     // MS Validated
            retloc: false,  // Retrieve Location Indication Flag
            pbic: false,    // Propagate BBAI Information Change
            srni: false,    // SGW Restoration Needed Indication
            s6af: false,    // Static IPv6 Address Flag
            s4af: false,    // Static IPv4 Address Flag
            mbmdt: false,   // Management Based MDT allowed flag
            israu: false,   // ISR is activated for the UE
            ccrsi: false,   // CSG Change Reporting Support Indication
            cprai: false,   // Change of Presence Reporting Area information Indication
            arrl: false,    // Abnormal Release of Radio Link
            ppoff: false,   // PDN Pause Off Indication
            ppon: false,    // PDN Pause On Indication
            ppsi: false,    // PDN Pause Support Indication
            csfbi: false,   // CSFB Indication
            clii: false,    // Change of Location Information Indication
            cpsr: false,    // CS to PS SRVCC indication
            nsi: false,     // NBIFOM Support Indication
            uasi: false,    // UE Available for Signalling Indication
            dtci: false,    // Delay Tolerant Connection Indication
            bdwi: false,    // Buffered DL Data Waiting Indication
            psci: false,    // Pending Subscription Change Indication
            pcri: false,    // P-CSCF Restoration Indication
            aosi: false,    // Associate OCI with SGW node's Identity
            aopi: false,    // Associate OCI with PGW node's Identity
            roaai: false,   // Release Over Any Access Indication
            epcosi: false,  // Extended PCO Support Indication
            cpopci: false,  // Control Plane Only PDN Connection Indication
            pmtsmi: false,  // Pending MT Short Message Indication
            s11tf: false,   // S11-U Tunnel Flag
            pnsi: false,    // Pending Network Initiated PDN Connection Signalling Indication
            unaccsi: false, // UE Not Authorised Cause Code Support Indication
            wpmsi: false,   // WLCP PDN Connection Modification Support Indication
            g5snn26: false, // 5GS Interworking without N26 indication
            reprefi: false, // Return Preferred Indication
            g5siwki: false, // 5GS Interworking Indication
            eevrsi: false,  // Extended EBI Value Range Support Indication
            ltemui: false,  // LTE-M UE Indication
            ltempi: false,  // LTE-M RAT Type reporting to PGW Indication
            enbcrsi: false, // eNB Change Reporting Support Indication
            tspcmi: false, // Triggering SGSN initiated PDP Context Creation/Modification Indication
            csrmfi: false, // Create Session Request Message Forwarded Indication
            mtedtn: false, // MT-EDT Not Applicable Indication
            mtedta: false, // MT-EDT Applicable Indication
            n5gnmi: false, // No 5GS N26 Mobility Indication
            g5cnrs: false, // 5GC Not Restricted Support Indication
            g5cnri: false, // 5GC Not Restricted Indication
            s5rho: false,  // 5G-SRVCC Handover Indication
            ethpdn: false, // Ethernet PDN Support Indication
            nspusi: false, // Notify Start Pause of charging via User plane Support Indication
            pgwrnsi: false, // PGW Redirection due to mismatch with Network Slice subscribed by UE Support Indication
            rppcsi: false, // Restoration of PDN connections after an PGW-C/SMF Change Support Indication
            pgwchi: false, // PGW CHange Indication
            sissme: false, // Same IWK-SCEF Selected for Monitoring Event Indication
            nsenbi: false, // Notify Source eNodeB Indication
            idfupf: false, // Indirect Data Forwarding with UPF Indication
            emci: false,   // Emergency PDU Session Indication
            ltemsai: false, // LTE-M Satellite Access Indication
            srtpi: false,  // Satellite RAT Type reporting to PGW Indication
            upipsi: false, // User Plane Integrity Protection Support Indication
        }
    }
}

impl From<Indication> for InformationElement {
    fn from(i: Indication) -> Self {
        InformationElement::Indication(i)
    }
}

impl From<&Indication> for Vec<u8> {
    fn from(i: &Indication) -> Vec<u8> {
        let buffer = vec![
            (i.daf as u8) << 7
                | (i.dtf as u8) << 6
                | (i.hi as u8) << 5
                | (i.dfi as u8) << 4
                | (i.oi as u8) << 3
                | (i.isrsi as u8) << 2
                | (i.israi as u8) << 1
                | (i.sgwci as u8),
            (i.sqci as u8) << 7
                | (i.uimsi as u8) << 6
                | (i.cfsi as u8) << 5
                | (i.crsi as u8) << 4
                | (i.ps as u8) << 3
                | (i.pt as u8) << 2
                | (i.si as u8) << 1
                | (i.msv as u8),
            (i.retloc as u8) << 7
                | (i.pbic as u8) << 6
                | (i.srni as u8) << 5
                | (i.s6af as u8) << 4
                | (i.s4af as u8) << 3
                | (i.mbmdt as u8) << 2
                | (i.israu as u8) << 1
                | (i.ccrsi as u8),
            (i.cprai as u8) << 7
                | (i.arrl as u8) << 6
                | (i.ppoff as u8) << 5
                | (i.ppon as u8) << 4
                | (i.ppsi as u8) << 3
                | (i.csfbi as u8) << 2
                | (i.clii as u8) << 1
                | (i.cpsr as u8),
            (i.nsi as u8) << 7
                | (i.uasi as u8) << 6
                | (i.dtci as u8) << 5
                | (i.bdwi as u8) << 4
                | (i.psci as u8) << 3
                | (i.pcri as u8) << 2
                | (i.aosi as u8) << 1
                | (i.aopi as u8),
            (i.roaai as u8) << 7
                | (i.epcosi as u8) << 6
                | (i.cpopci as u8) << 5
                | (i.pmtsmi as u8) << 4
                | (i.s11tf as u8) << 3
                | (i.pnsi as u8) << 2
                | (i.unaccsi as u8) << 1
                | (i.wpmsi as u8),
            (i.g5snn26 as u8) << 7
                | (i.reprefi as u8) << 6
                | (i.g5siwki as u8) << 5
                | (i.eevrsi as u8) << 4
                | (i.ltemui as u8) << 3
                | (i.ltempi as u8) << 2
                | (i.enbcrsi as u8) << 1
                | (i.tspcmi as u8),
            (i.csrmfi as u8) << 7
                | (i.mtedtn as u8) << 6
                | (i.mtedta as u8) << 5
                | (i.n5gnmi as u8) << 4
                | (i.g5cnrs as u8) << 3
                | (i.g5cnri as u8) << 2
                | (i.s5rho as u8) << 1
                | (i.ethpdn as u8),
            (i.nspusi as u8) << 7
                | (i.pgwrnsi as u8) << 6
                | (i.rppcsi as u8) << 5
                | (i.pgwchi as u8) << 4
                | (i.sissme as u8) << 3
                | (i.nsenbi as u8) << 2
                | (i.idfupf as u8) << 1
                | (i.emci as u8),
            (i.ltemsai as u8) << 2 | (i.srtpi as u8) << 1 | (i.upipsi as u8),
        ];
        buffer
    }
}

impl Indication {
    pub fn convert(&mut self, buffer: Vec<u8>) {
        for i in buffer.iter().enumerate() {
            match i {
                (0, _) => {
                    self.daf = (i.1 & 0x80) >> 7 == 1;
                    self.dtf = (i.1 & 0x40) >> 6 == 1;
                    self.hi = (i.1 & 0x20) >> 5 == 1;
                    self.dfi = (i.1 & 0x10) >> 4 == 1;
                    self.oi = (i.1 & 0x08) >> 3 == 1;
                    self.isrsi = (i.1 & 0x04) >> 2 == 1;
                    self.israi = (i.1 & 0x02) >> 1 == 1;
                    self.sgwci = (i.1 & 0x01) == 1;
                }
                (1, _) => {
                    self.sqci = (i.1 & 0x80) >> 7 == 1;
                    self.uimsi = (i.1 & 0x40) >> 6 == 1;
                    self.cfsi = (i.1 & 0x20) >> 5 == 1;
                    self.crsi = (i.1 & 0x10) >> 4 == 1;
                    self.ps = (i.1 & 0x08) >> 3 == 1;
                    self.pt = (i.1 & 0x04) >> 2 == 1;
                    self.si = (i.1 & 0x02) >> 1 == 1;
                    self.msv = (i.1 & 0x01) == 1;
                }
                (2, _) => {
                    self.retloc = (i.1 & 0x80) >> 7 == 1;
                    self.pbic = (i.1 & 0x40) >> 6 == 1;
                    self.srni = (i.1 & 0x20) >> 5 == 1;
                    self.s6af = (i.1 & 0x10) >> 4 == 1;
                    self.s4af = (i.1 & 0x08) >> 3 == 1;
                    self.mbmdt = (i.1 & 0x04) >> 2 == 1;
                    self.israu = (i.1 & 0x02) >> 1 == 1;
                    self.ccrsi = (i.1 & 0x01) == 1;
                }
                (3, _) => {
                    self.cprai = (i.1 & 0x80) >> 7 == 1;
                    self.arrl = (i.1 & 0x40) >> 6 == 1;
                    self.ppoff = (i.1 & 0x20) >> 5 == 1;
                    self.ppon = (i.1 & 0x10) >> 4 == 1;
                    self.ppsi = (i.1 & 0x08) >> 3 == 1;
                    self.csfbi = (i.1 & 0x04) >> 2 == 1;
                    self.clii = (i.1 & 0x02) >> 1 == 1;
                    self.cpsr = (i.1 & 0x01) == 1;
                }
                (4, _) => {
                    self.nsi = (i.1 & 0x80) >> 7 == 1;
                    self.uasi = (i.1 & 0x40) >> 6 == 1;
                    self.dtci = (i.1 & 0x20) >> 5 == 1;
                    self.bdwi = (i.1 & 0x10) >> 4 == 1;
                    self.psci = (i.1 & 0x08) >> 3 == 1;
                    self.pcri = (i.1 & 0x04) >> 2 == 1;
                    self.aosi = (i.1 & 0x02) >> 1 == 1;
                    self.aopi = (i.1 & 0x01) == 1;
                }
                (5, _) => {
                    self.roaai = (i.1 & 0x80) >> 7 == 1;
                    self.epcosi = (i.1 & 0x40) >> 6 == 1;
                    self.cpopci = (i.1 & 0x20) >> 5 == 1;
                    self.pmtsmi = (i.1 & 0x10) >> 4 == 1;
                    self.s11tf = (i.1 & 0x08) >> 3 == 1;
                    self.pnsi = (i.1 & 0x04) >> 2 == 1;
                    self.unaccsi = (i.1 & 0x02) >> 1 == 1;
                    self.wpmsi = (i.1 & 0x01) == 1;
                }
                (6, _) => {
                    self.g5snn26 = (i.1 & 0x80) >> 7 == 1;
                    self.reprefi = (i.1 & 0x40) >> 6 == 1;
                    self.g5siwki = (i.1 & 0x20) >> 5 == 1;
                    self.eevrsi = (i.1 & 0x10) >> 4 == 1;
                    self.ltemui = (i.1 & 0x08) >> 3 == 1;
                    self.ltempi = (i.1 & 0x04) >> 2 == 1;
                    self.enbcrsi = (i.1 & 0x02) >> 1 == 1;
                    self.tspcmi = (i.1 & 0x01) == 1;
                }
                (7, _) => {
                    self.csrmfi = (i.1 & 0x80) >> 7 == 1;
                    self.mtedtn = (i.1 & 0x40) >> 6 == 1;
                    self.mtedta = (i.1 & 0x20) >> 5 == 1;
                    self.n5gnmi = (i.1 & 0x10) >> 4 == 1;
                    self.g5cnrs = (i.1 & 0x08) >> 3 == 1;
                    self.g5cnri = (i.1 & 0x04) >> 2 == 1;
                    self.s5rho = (i.1 & 0x02) >> 1 == 1;
                    self.ethpdn = (i.1 & 0x01) == 1;
                }
                (8, _) => {
                    self.nspusi = (i.1 & 0x80) >> 7 == 1;
                    self.pgwrnsi = (i.1 & 0x40) >> 6 == 1;
                    self.rppcsi = (i.1 & 0x20) >> 5 == 1;
                    self.pgwchi = (i.1 & 0x10) >> 4 == 1;
                    self.sissme = (i.1 & 0x08) >> 3 == 1;
                    self.nsenbi = (i.1 & 0x04) >> 2 == 1;
                    self.idfupf = (i.1 & 0x02) >> 1 == 1;
                    self.emci = (i.1 & 0x01) == 1;
                }
                (9, _) => {
                    self.ltemsai = (i.1 & 0x04) >> 2 == 1;
                    self.srtpi = (i.1 & 0x02) >> 1 == 1;
                    self.upipsi = (i.1 & 0x01) == 1;
                }
                _ => (),
            }
        }
    }
}

impl IEs for Indication {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(INDICATION);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let mut flags: Vec<u8> = self.into();
        buffer_ie.append(&mut flags);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE {
            let mut data = Indication {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                ..Indication::default()
            };
            data.convert(buffer[MIN_IE_SIZE..(data.length as usize) + MIN_IE_SIZE].to_vec());
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(INDICATION))
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
fn indication_ie_marshal_test() {
    let encoded: [u8; 14] = [
        0x4d, 0x00, 0x0a, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x04,
    ];
    let decoded = Indication {
        tspcmi: true,
        sgwci: true,
        ltemsai: true,
        ..Default::default()
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn indication_ie_unmarshal_test() {
    let encoded: [u8; 14] = [
        0x4d, 0x00, 0x0a, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x04,
    ];
    let decoded = Indication {
        tspcmi: true,
        sgwci: true,
        ltemsai: true,
        ..Indication::default()
    };
    assert_eq!(Indication::unmarshal(&encoded).unwrap(), decoded);
}

#[test]
fn indication_ie_unmarshal_legacy_test() {
    let encoded: [u8; 11] = [
        0x4d, 0x00, 0x07, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
    ];
    let decoded = Indication {
        length: 7,
        tspcmi: true,
        sgwci: true,
        ..Indication::default()
    };
    assert_eq!(Indication::unmarshal(&encoded).unwrap(), decoded);
}
