// Indication IE - according to 3GPP TS 29.274 V15.9.0 (2019-09) 

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*, ie::*}};

// Indication IE TL

pub const INDICATION:u8 = 77;
pub const INDICATION_LENGTH:usize = 7;

// Indication IE implementation

#[derive(Debug, Clone, PartialEq)]
pub struct Indication {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub daf:bool,                       // Dual Address Bearer Flag
    pub dtf:bool,                       // Direct Tunnel Flag
    pub hi:bool,                        // Handover Indication
    pub dfi:bool,                       // Direct Forwarding Indication
    pub oi:bool,                        // Operation Indication
    pub isrsi:bool,                     // Idle mode Signalling Reduction Supported Indication
    pub israi:bool,                     // Idle mode Signalling Reduction Activation Indication
    pub sgwci:bool,                     // SGW Change Indication
    pub sqci:bool,                      // Subscribed QoS Change Indication
    pub uimsi:bool,                     // Unauthenticated IMSI
    pub cfsi:bool,                      // Change F-TEID support Indication
    pub crsi:bool,                      // Change Reporting Support Indication
    pub ps:bool,                        // Piggybacking Supported
    pub pt:bool,                        // S5/S8 Protocol Type
    pub si:bool,                        // Scope Indication
    pub msv:bool,                       // MS Validated
    pub retloc:bool,                    // Retrieve Location Indication Flag
    pub pbic:bool,                      // Propagate BBAI Information Change
    pub srni:bool,                      // SGW Restoration Needed Indication
    pub s6af:bool,                      // Static IPv6 Address Flag
    pub s4af:bool,                      // Static IPv4 Address Flag
    pub mbmdt:bool,                     // Management Based MDT allowed flag
    pub israu:bool,                     // ISR is activated for the UE
    pub ccrsi:bool,                     // CSG Change Reporting Support Indication
    pub cprai:bool,                     // Change of Presence Reporting Area information Indication
    pub arrl:bool,                      // Abnormal Release of Radio Link
    pub ppoff:bool,                     // PDN Pause Off Indication
    pub ppon:bool,                      // PDN Pause On Indication
    pub ppsi:bool,                      // PDN Pause Support Indication
    pub csfbi:bool,                     // CSFB Indication
    pub clii:bool,                      // Change of Location Information Indication
    pub cpsr:bool,                      // CS to PS SRVCC indication
    pub nsi:bool,                       // NBIFOM Support Indication
    pub uasi:bool,                      // UE Available for Signalling Indication
    pub dtci:bool,                      // Delay Tolerant Connection Indication
    pub bdwi:bool,                      // Buffered DL Data Waiting Indication
    pub psci:bool,                      // Pending Subscription Change Indication
    pub pcri:bool,                      // P-CSCF Restoration Indication
    pub aosi:bool,                      // Associate OCI with SGW node's Identity
    pub aopi:bool,                      // Associate OCI with PGW node's Identity
    pub roaai:bool,                     // Release Over Any Access Indication
    pub epcosi:bool,                    // Extended PCO Support Indication
    pub cpopci:bool,                    // Control Plane Only PDN Connection Indication
    pub pmtsmi:bool,                    // Pending MT Short Message Indication
    pub s11tf:bool,                     // S11-U Tunnel Flag
    pub pnsi:bool,                      // Pending Network Initiated PDN Connection Signalling Indication
    pub unaccsi:bool,                   // UE Not Authorised Cause Code Support Indication
    pub wpmsi:bool,                     // WLCP PDN Connection Modification Support Indication
    pub g5snn26:bool,                   // 5GS Interworking without N26 indication
    pub reprefi:bool,                   // Return Preferred Indication
    pub g5siwki:bool,                   // 5GS Interworking Indication
    pub eevrsi:bool,                    // Extended EBI Value Range Support Indication
    pub ltemui:bool,                    // LTE-M UE Indication
    pub ltempi:bool,                    // LTE-M RAT Type reporting to PGW Indication
    pub enbcrsi:bool,                   // eNB Change Reporting Support Indication
    pub tspcmi:bool,                    // Triggering SGSN initiated PDP Context Creation/Modification Indication
}

impl Default for Indication {
    fn default() -> Self {
        Indication { 
                    t: INDICATION,
                    length:INDICATION_LENGTH as u16,
                    ins: 0,
                    daf:false,                       // Dual Address Bearer Flag
                    dtf:false,                       // Direct Tunnel Flag
                    hi:false,                        // Handover Indication
                    dfi:false,                       // Direct Forwarding Indication
                    oi:false,                        // Operation Indication
                    isrsi:false,                     // Idle mode Signalling Reduction Supported Indication
                    israi:false,                     // Idle mode Signalling Reduction Activation Indication
                    sgwci:false,                     // SGW Change Indication
                    sqci:false,                      // Subscribed QoS Change Indication
                    uimsi:false,                     // Unauthenticated IMSI
                    cfsi:false,                      // Change F-TEID support Indication
                    crsi:false,                      // Change Reporting Support Indication
                    ps:false,                        // Piggybacking Supported
                    pt:false,                        // S5/S8 Protocol Type
                    si:false,                        // Scope Indication
                    msv:false,                       // MS Validated
                    retloc:false,                    // Retrieve Location Indication Flag
                    pbic:false,                      // Propagate BBAI Information Change
                    srni:false,                      // SGW Restoration Needed Indication
                    s6af:false,                      // Static IPv6 Address Flag
                    s4af:false,                      // Static IPv4 Address Flag
                    mbmdt:false,                     // Management Based MDT allowed flag
                    israu:false,                     // ISR is activated for the UE
                    ccrsi:false,                     // CSG Change Reporting Support Indication
                    cprai:false,                     // Change of Presence Reporting Area information Indication
                    arrl:false,                      // Abnormal Release of Radio Link
                    ppoff:false,                     // PDN Pause Off Indication
                    ppon:false,                      // PDN Pause On Indication
                    ppsi:false,                      // PDN Pause Support Indication
                    csfbi:false,                     // CSFB Indication
                    clii:false,                      // Change of Location Information Indication
                    cpsr:false,                      // CS to PS SRVCC indication
                    nsi:false,                       // NBIFOM Support Indication
                    uasi:false,                      // UE Available for Signalling Indication
                    dtci:false,                      // Delay Tolerant Connection Indication
                    bdwi:false,                      // Buffered DL Data Waiting Indication
                    psci:false,                      // Pending Subscription Change Indication
                    pcri:false,                      // P-CSCF Restoration Indication
                    aosi:false,                      // Associate OCI with SGW node's Identity
                    aopi:false,                      // Associate OCI with PGW node's Identity
                    roaai:false,                     // Release Over Any Access Indication
                    epcosi:false,                    // Extended PCO Support Indication
                    cpopci:false,                    // Control Plane Only PDN Connection Indication
                    pmtsmi:false,                    // Pending MT Short Message Indication
                    s11tf:false,                     // S11-U Tunnel Flag
                    pnsi:false,                      // Pending Network Initiated PDN Connection Signalling Indication
                    unaccsi:false,                   // UE Not Authorised Cause Code Support Indication
                    wpmsi:false,                     // WLCP PDN Connection Modification Support Indication
                    g5snn26:false,                   // 5GS Interworking without N26 indication
                    reprefi:false,                   // Return Preferred Indication
                    g5siwki:false,                   // 5GS Interworking Indication
                    eevrsi:false,                    // Extended EBI Value Range Support Indication
                    ltemui:false,                    // LTE-M UE Indication
                    ltempi:false,                    // LTE-M RAT Type reporting to PGW Indication
                    enbcrsi:false,                   // eNB Change Reporting Support Indication
                    tspcmi:false,                    // Triggering SGSN initiated PDP Context Creation/Modification Indication         
                }
    }
}

impl From<Indication> for InformationElement {
    fn from(i: Indication) -> Self {
        InformationElement::Indication(i)
    }
}

impl IEs for Indication {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        let flags = self.clone().into_array().iter().map( |x| if *x {1} else {0}).enumerate().map( |(i,x)| x<<(55-i)).collect::<Vec<_>>().iter().sum::<u64>();
        let i = flags.to_be_bytes();
        buffer_ie.extend_from_slice(&i[1..]);
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> where Self:Sized {
        if buffer.len()>=INDICATION_LENGTH {
            let mut data=Indication::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            let f = u64::from_be_bytes([0x00,buffer[4],buffer[5],buffer[6],buffer[7],buffer[8],buffer[9],buffer[10]]);
            let flags = [f;56].iter().enumerate().map(|(i,x)| if ((*x & (0x80000000000000 >> i))>>(55-i)) as u8 == 1 {true} else {false}).collect::<Vec<bool>>();
            data.from_array(&flags[..]);
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(INDICATION))
        }
    }

    fn len (&self) -> usize {
       (self.length+4) as usize 
    }

}

impl Indication {
    fn into_array(self) -> [bool;56] {
        [
            self.daf,                       // Dual Address Bearer Flag
            self.dtf,                       // Direct Tunnel Flag
            self.hi,                        // Handover Indication
            self.dfi,                       // Direct Forwarding Indication
            self.oi,                        // Operation Indication
            self.isrsi,                     // Idle mode Signalling Reduction Supported Indication
            self.israi,                     // Idle mode Signalling Reduction Activation Indication
            self.sgwci,                     // SGW Change Indication
            self.sqci,                      // Subscribed QoS Change Indication
            self.uimsi,                     // Unauthenticated IMSI
            self.cfsi,                      // Change F-TEID support Indication
            self.crsi,                      // Change Reporting Support Indication
            self.ps,                        // Piggybacking Supported
            self.pt,                        // S5/S8 Protocol Type
            self.si,                        // Scope Indication
            self.msv,                       // MS Validated
            self.retloc,                    // Retrieve Location Indication Flag
            self.pbic,                      // Propagate BBAI Information Change
            self.srni,                      // SGW Restoration Needed Indication
            self.s6af,                      // Static IPv6 Address Flag
            self.s4af,                      // Static IPv4 Address Flag
            self.mbmdt,                     // Management Based MDT allowed flag
            self.israu,                     // ISR is activated for the UE
            self.ccrsi,                     // CSG Change Reporting Support Indication
            self.cprai,                     // Change of Presence Reporting Area information Indication
            self.arrl,                      // Abnormal Release of Radio Link
            self.ppoff,                     // PDN Pause Off Indication
            self.ppon,                      // PDN Pause On Indication
            self.ppsi,                      // PDN Pause Support Indication
            self.csfbi,                     // CSFB Indication
            self.clii,                      // Change of Location Information Indication
            self.cpsr,                      // CS to PS SRVCC indication
            self.nsi,                       // NBIFOM Support Indication
            self.uasi,                      // UE Available for Signalling Indication
            self.dtci,                      // Delay Tolerant Connection Indication
            self.bdwi,                      // Buffered DL Data Waiting Indication
            self.psci,                      // Pending Subscription Change Indication
            self.pcri,                      // P-CSCF Restoration Indication
            self.aosi,                      // Associate OCI with SGW node's Identity
            self.aopi,                      // Associate OCI with PGW node's Identity
            self.roaai,                     // Release Over Any Access Indication
            self.epcosi,                    // Extended PCO Support Indication
            self.cpopci,                    // Control Plane Only PDN Connection Indication
            self.pmtsmi,                    // Pending MT Short Message Indication
            self.s11tf,                     // S11-U Tunnel Flag
            self.pnsi,                      // Pending Network Initiated PDN Connection Signalling Indication
            self.unaccsi,                   // UE Not Authorised Cause Code Support Indication
            self.wpmsi,                     // WLCP PDN Connection Modification Support Indication
            self.g5snn26,                   // 5GS Interworking without N26 indication
            self.reprefi,                   // Return Preferred Indication
            self.g5siwki,                   // 5GS Interworking Indication
            self.eevrsi,                    // Extended EBI Value Range Support Indication
            self.ltemui,                    // LTE-M UE Indication
            self.ltempi,                    // LTE-M RAT Type reporting to PGW Indication
            self.enbcrsi,                   // eNB Change Reporting Support Indication
            self.tspcmi                     // Triggering SGSN initiated PDP Context Creation/Modification Indication
        ]
    }
    fn from_array(&mut self, i:&[bool]) {
            self.daf = i[0];                       // Dual Address Bearer Flag
            self.dtf = i[1];                       // Direct Tunnel Flag
            self.hi = i[2];                        // Handover Indication
            self.dfi = i[3];                       // Direct Forwarding Indication
            self.oi = i[4];                        // Operation Indication
            self.isrsi = i[5];                     // Idle mode Signalling Reduction Supported Indication
            self.israi = i[6];                     // Idle mode Signalling Reduction Activation Indication
            self.sgwci = i[7];                     // SGW Change Indication
            self.sqci = i[8];                      // Subscribed QoS Change Indication
            self.uimsi = i[9];                     // Unauthenticated IMSI
            self.cfsi = i[10];                      // Change F-TEID support Indication
            self.crsi = i[11];                      // Change Reporting Support Indication
            self.ps = i[12];                        // Piggybacking Supported
            self.pt = i[13];                        // S5/S8 Protocol Type
            self.si = i[14];                        // Scope Indication
            self.msv = i[15];                       // MS Validated
            self.retloc = i[16];                    // Retrieve Location Indication Flag
            self.pbic = i[17];                      // Propagate BBAI Information Change
            self.srni = i[18];                      // SGW Restoration Needed Indication
            self.s6af = i[19];                      // Static IPv6 Address Flag
            self.s4af = i[20];                      // Static IPv4 Address Flag
            self.mbmdt = i[21];                     // Management Based MDT allowed flag
            self.israu = i[22];                     // ISR is activated for the UE
            self.ccrsi = i[23];                     // CSG Change Reporting Support Indication
            self.cprai = i[24];                     // Change of Presence Reporting Area information Indication
            self.arrl = i[25];                      // Abnormal Release of Radio Link
            self.ppoff = i[26];                     // PDN Pause Off Indication
            self.ppon = i[27];                      // PDN Pause On Indication
            self.ppsi = i[28];                      // PDN Pause Support Indication
            self.csfbi = i[29];                     // CSFB Indication
            self.clii = i[30];                      // Change of Location Information Indication
            self.cpsr = i[31];                      // CS to PS SRVCC indication
            self.nsi = i[32];                       // NBIFOM Support Indication
            self.uasi = i[33];                      // UE Available for Signalling Indication
            self.dtci = i[34];                      // Delay Tolerant Connection Indication
            self.bdwi = i[35];                      // Buffered DL Data Waiting Indication
            self.psci = i[36];                      // Pending Subscription Change Indication
            self.pcri = i[37];                      // P-CSCF Restoration Indication
            self.aosi = i[38];                      // Associate OCI with SGW node's Identity
            self.aopi = i[39];                      // Associate OCI with PGW node's Identity
            self.roaai = i[40];                     // Release Over Any Access Indication
            self.epcosi = i[41];                    // Extended PCO Support Indication
            self.cpopci = i[42];                    // Control Plane Only PDN Connection Indication
            self.pmtsmi = i[43];                    // Pending MT Short Message Indication
            self.s11tf = i[44];                     // S11-U Tunnel Flag
            self.pnsi = i[45];                      // Pending Network Initiated PDN Connection Signalling Indication
            self.unaccsi = i[46];                   // UE Not Authorised Cause Code Support Indication
            self.wpmsi = i[47];                     // WLCP PDN Connection Modification Support Indication
            self.g5snn26 = i[48];                   // 5GS Interworking without N26 indication
            self.reprefi = i[49];                   // Return Preferred Indication
            self.g5siwki = i[50];                   // 5GS Interworking Indication
            self.eevrsi = i[51];                    // Extended EBI Value Range Support Indication
            self.ltemui = i[52];                    // LTE-M UE Indication
            self.ltempi = i[53];                    // LTE-M RAT Type reporting to PGW Indication
            self.enbcrsi = i[54];                   // eNB Change Reporting Support Indication
            self.tspcmi = i[55];                    // Triggering SGSN initiated PDP Context Creation/Modification Indication
    }
}

#[test]
fn indication_ie_marshal_test () {
    let encoded:[u8;11]=[0x4d, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let decoded = Indication::default();
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}

#[test]
fn indication_ie_unmarshal_test () {
    let encoded:[u8;11]=[0x4d, 0x00, 0x07, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
    let mut decoded = Indication::default();
    decoded.tspcmi = true;
    decoded.sgwci = true;
    assert_eq!(Indication::unmarshal(&encoded).unwrap(), decoded);
}