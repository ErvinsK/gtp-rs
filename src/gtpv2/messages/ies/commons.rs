// Commons for GTP-C IEs

use crate::gtpv2::{errors::GTPV2Error, messages::ies::*};

pub const MIN_IE_SIZE:usize = 4;

pub trait IEs {
    fn marshal (&self, buffer: &mut Vec<u8>);
    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> where Self:Sized;
    fn len (&self) -> usize; // Total IE length = Type+Length+Instance+Value for TLIV messages
}

#[derive(Debug, Clone, PartialEq)]
pub enum InformationElement {
    Imsi(Imsi),
    Cause(Cause),
    Recovery(Recovery),
    // STN-SR
    Apn(Apn),
    ApnAmbr(ApnAmbr),
    Ebi(Ebi),
    IpAddress(IpAddress),
    Mei(Mei),
    Msisdn(Msisdn),
    Indication(Indication),
    Pco(Pco),
    PdnAddressAllocation(PdnAddressAllocation),
    BearerQos(BearerQos),
    FlowQos(FlowQos),
    RatType(RatType),
    ServingNetwork(ServingNetwork),
    BearerTft(BearerTft),
    TrafficAggregateDescription(TrafficAggregateDescription),
    Uli(Uli),
    Fteid(Fteid),
    // Tmsi(Tmsi),
    // Global CN-id
    // S103 PDN Data Forwarding Info
    // S1-U Data Forwarding Info
    // Delay Value
    // Bearer Context
    ChargingId(ChargingId),
    ChargingCharacteristics(ChargingCharacteristics),
    TraceInformation(TraceInformation),
    // Bearer Flags
    PdnType(PdnType),
    // Procedure Transaction ID
    // MM Context (GSM Keys and Triplets)
    // MM Context (UMTS Keys, Used Chiper, and Quintuplets)
    // MM Context (GSM Keys, Used Chiper, and Quintuplets)
    // MM Context (UMTS Keys and Quintuplets)
    // MM Context (EPS Security Context, Quadruplets and Quintuplets)
    // MM Context (UMTS Key, Quadruplets and Quintuplets)
    // PDN Connection
    // PDU Numbers
    Ptmsi(Ptmsi),
    PtmsiSignature(PtmsiSignature),
    HopCounter(HopCounter),
    UeTimeZone(UeTimeZone),
    TraceReference(TraceReference),
    // Complete Request Message
    // GUTI
    // F-Container
    // F-Cause
    PlmnId(PlmnId),
    // Target Identification
    PacketFlowId(PacketFlowId),
    // RAB Context
    // Source RNC PDCP Context Info
    PortNumber(PortNumber),
    ApnRestriction(ApnRestriction),
    SelectionMode(SelectionMode),
    // Source Identification
    ChangeReportingAction(ChangeReportingAction),
    // FQ-CSID
    // Channel Needed
    // eMLPP Priority
    NodeType(NodeType),
    // FQDN
    // Transaction Identifier
    // MBMS Session Duration
    // MBMS Service Area
    // MBMS Session Identifier
    // MBMS Flow Identifier
    // MBMS IP Multicast Distribution
    // MBMS Distribution Acknowledge
    // RFSP Index
    Uci(Uci),
    CSGInformationReportingAction(CSGInformationReportingAction),
    // CSG ID
    // CSG Membership Indication
    // Service Indicator
    // Detach Type
    // Local Distiguished Name
    NodeFeatures(NodeFeatures),
    // MBMS Time to Data Transfer
    // Throttling
    // Allocation/Retention Priority
    // EPC Timer
    // Signalling Priority Indication
    // Temporary Mobile Group Identity
    // Additional MM context SRVCC
    // Additional flags SRVCC
    // MDT Configuration
    // Additional PCO
    // Absolute Time of MBMS Data Transfer
    // H(e)NB Information Reporting
    // IPv4 Configuration Parameters (IP4CP)
    // Change to Report Flags
    // Action Indication
    // TWAN Identifier
    // ULI Timestamp
    // MBMS Flags
    // RAN/NAS Cause
    // CN Operator Selection Entity
    // Trusted WLAN Mode Indication
    // Node Number
    // Node Identifier
    // Presence Reporting Area Action
    // Presence Reporting Area Information
    // TWAN Identifier Timestamp
    // Overload Control Information
    // Load Control Information
    // Metric
    // Sequence Number
    // APN and Relative Capacity
    // WLAN Offloadability Indication
    // Paging and Service Information
    // Integer Number
    // Millisecond Time Stamp
    // Monitoring Event Information
    // ECGI List
    // Remote UE Context
    // Remote User ID
    // Remote UE IP Information
    // CIoT Optimization Support Indication
    // SCEF PDN Connection
    // Header Compression Configuration
    // Extended PCO
    // Serving PLMN Rate Control
    // Counter
    // Mapped UE Usage Type
    // Secondary UE Usage Type
    // UP Function Selection Indication Flags
    // Max Packet Loss Rate
    // APN Rate Control Status
    // Extended Trace Information
    // Monitoring Event Extension Information
    // Special IE type for IE Type Extension
    PrivateExtension(PrivateExtension),
}

impl InformationElement {
    pub fn marshal (self, buffer: &mut Vec<u8>) {
        match self {
            InformationElement::Imsi(i) => i.marshal(buffer),
            InformationElement::Cause(i) => i.marshal(buffer),
            InformationElement::Recovery(i) => i.marshal(buffer),
            // STN-SR
            InformationElement::Apn(i) => i.marshal(buffer),
            InformationElement::ApnAmbr(i) => i.marshal(buffer),
            InformationElement::Ebi(i) => i.marshal(buffer),
            InformationElement::IpAddress(i) => i.marshal(buffer),
            InformationElement::Mei(i) => i.marshal(buffer),
            InformationElement::Msisdn(i) => i.marshal(buffer),
            InformationElement::Indication(i) => i.marshal(buffer),
            InformationElement::Pco(i) => i.marshal(buffer),
            InformationElement::PdnAddressAllocation(i) => i.marshal(buffer),
            InformationElement::BearerQos(i) => i.marshal(buffer),
            InformationElement::FlowQos(i) => i.marshal(buffer),
            InformationElement::RatType(i) => i.marshal(buffer),
            InformationElement::ServingNetwork(i) => i.marshal(buffer),
            InformationElement::BearerTft(i) => i.marshal(buffer),
            InformationElement::TrafficAggregateDescription(i) => i.marshal(buffer),
            InformationElement::Uli(i) => i.marshal(buffer),
            InformationElement::Fteid(i) => i.marshal(buffer),
            // Tmsi(Tmsi),
            // Global CN-id
            // S103 PDN Data Forwarding Info
            // S1-U Data Forwarding Info
            // Delay Value
            // Bearer Context
            InformationElement::ChargingId(i) => i.marshal(buffer),
            InformationElement::ChargingCharacteristics(i) => i.marshal(buffer),
            InformationElement::TraceInformation(i) => i.marshal(buffer),
            // Bearer Flags
            InformationElement::PdnType(i) => i.marshal(buffer),
            // Procedure Transaction ID
            // MM Context (GSM Keys and Triplets)
            // MM Context (UMTS Keys, Used Chiper, and Quintuplets)
            // MM Context (GSM Keys, Used Chiper, and Quintuplets)
            // MM Context (UMTS Keys and Quintuplets)
            // MM Context (EPS Security Context, Quadruplets and Quintuplets)
            // MM Context (UMTS Key, Quadruplets and Quintuplets)
            // PDN Connection
            // PDU Numbers
            InformationElement::Ptmsi(i) => i.marshal(buffer),
            InformationElement::PtmsiSignature(i) => i.marshal(buffer),
            InformationElement::HopCounter(i) => i.marshal(buffer),
            InformationElement::UeTimeZone(i) => i.marshal(buffer),
            InformationElement::TraceReference(i) => i.marshal(buffer),
            // Complete Request Message
            // GUTI
            // F-Container
            // F-Cause
            InformationElement::PlmnId(i) => i.marshal(buffer),
            // Target Identification
            InformationElement::PacketFlowId(i) => i.marshal(buffer),
            // RAB Context
            // Source RNC PDCP Context Info
            InformationElement::PortNumber(i) => i.marshal(buffer),
            InformationElement::ApnRestriction(i) => i.marshal(buffer),
            InformationElement::SelectionMode(i) => i.marshal(buffer),
            // Source Identification
            InformationElement::ChangeReportingAction(i) => i.marshal(buffer),
            // FQ-CSID
            // Channel Needed
            // eMLPP Priority
            InformationElement::NodeType(i) => i.marshal(buffer),
            // FQDN
            // Transaction Identifier
            // MBMS Session Duration
            // MBMS Service Area
            // MBMS Session Identifier
            // MBMS Flow Identifier
            // MBMS IP Multicast Distribution
            // MBMS Distribution Acknowledge
            // RFSP Index
            InformationElement::Uci(i) => i.marshal(buffer),
            InformationElement::CSGInformationReportingAction(i) => i.marshal(buffer),
            // CSG ID
            // CSG Membership Indication
            // Service Indicator
            // Detach Type
            // Local Distiguished Name
            InformationElement::NodeFeatures(i) => i.marshal(buffer),
            // MBMS Time to Data Transfer
            // Throttling
            // Allocation/Retention Priority
            // EPC Timer
            // Signalling Priority Indication
            // Temporary Mobile Group Identity
            // Additional MM context SRVCC
            // Additional flags SRVCC
            // MDT Configuration
            // Additional PCO
            // Absolute Time of MBMS Data Transfer
            // H(e)NB Information Reporting
            // IPv4 Configuration Parameters (IP4CP)
            // Change to Report Flags
            // Action Indication
            // TWAN Identifier
            // ULI Timestamp
            // MBMS Flags
            // RAN/NAS Cause
            // CN Operator Selection Entity
            // Trusted WLAN Mode Indication
            // Node Number
            // Node Identifier
            // Presence Reporting Area Action
            // Presence Reporting Area Information
            // TWAN Identifier Timestamp
            // Overload Control Information
            // Load Control Information
            // Metric
            // Sequence Number
            // APN and Relative Capacity
            // WLAN Offloadability Indication
            // Paging and Service Information
            // Integer Number
            // Millisecond Time Stamp
            // Monitoring Event Information
            // ECGI List
            // Remote UE Context
            // Remote User ID
            // Remote UE IP Information
            // CIoT Optimization Support Indication
            // SCEF PDN Connection
            // Header Compression Configuration
            // Extended PCO
            // Serving PLMN Rate Control
            // Counter
            // Mapped UE Usage Type
            // Secondary UE Usage Type
            // UP Function Selection Indication Flags
            // Max Packet Loss Rate
            // APN Rate Control Status
            // Extended Trace Information
            // Monitoring Event Extension Information
            // Special IE type for IE Type Extension
            InformationElement::PrivateExtension(i) => i.marshal(buffer),  
        }
    }

    pub fn unmarshal_ie(buffer:&[u8]) -> Result<InformationElement, GTPV2Error> {
        match buffer[0] {
            1 => {
                match Imsi::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::Imsi(i)),
                    Err(j) => Err(j),
                }
            },
            2 => {
                match Cause::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::Cause(i)),
                    Err(j) => Err(j),
                }
            }, 
            3 => {
                match Recovery::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::Recovery(i)),
                    Err(j) => Err(j),
                }
            },
            // STN-SR   
            71 => {
                match Apn::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::Apn(i)),
                    Err(j) => Err(j),
                }
            },
            72 => {
                match ApnAmbr::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::ApnAmbr(i)),
                    Err(j) => Err(j),
                }
            },
            73 => {
                match Ebi::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::Ebi(i)),
                    Err(j) => Err(j),
                }
            },
            74 => {
                match IpAddress::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::IpAddress(i)),
                    Err(j) => Err(j),
                }
            },
            75 => {
                match Mei::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::Mei(i)),
                    Err(j) => Err(j),
                }
            },
            76 => {
                match Msisdn::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::Msisdn(i)),
                    Err(j) => Err(j),
                }
            },
            77 => {
                match Indication::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::Indication(i)),
                    Err(j) => Err(j),
                }
            },
            78 => {
                match Pco::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::Pco(i)),
                    Err(j) => Err(j),
                }
            },
            79 => {
                match PdnAddressAllocation::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::PdnAddressAllocation(i)),
                    Err(j) => Err(j),
                }
            },
            80 => {
                match BearerQos::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::BearerQos(i)),
                    Err(j) => Err(j),
                }
            },
            81 => {
                match FlowQos::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::FlowQos(i)),
                    Err(j) => Err(j),
                }
            },
            82 => {
                match RatType::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::RatType(i)),
                    Err(j) => Err(j),
                }
            },
            83 => {
                match ServingNetwork::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::ServingNetwork(i)),
                    Err(j) => Err(j),
                }
            },
            84 => {
                match BearerTft::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::BearerTft(i)),
                    Err(j) => Err(j),
                }
            },
            85 => {
                match TrafficAggregateDescription::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::TrafficAggregateDescription(i)),
                    Err(j) => Err(j),
                }
            },
            86 => {
                match Uli::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::Uli(i)),
                    Err(j) => Err(j),
                }
            },
            87 => {
                match Fteid::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::Fteid(i)),
                    Err(j) => Err(j),
                }
            },
            // Tmsi(Tmsi),
            // Global CN-id
            // S103 PDN Data Forwarding Info
            // S1-U Data Forwarding Info
            // Delay Value
            // Bearer Context
            94 => {
                match ChargingId::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::ChargingId(i)),
                    Err(j) => Err(j),
                }
            },
            95 => {
                match ChargingCharacteristics::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::ChargingCharacteristics(i)),
                    Err(j) => Err(j),
                }
            },
            96 => {
                match TraceInformation::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::TraceInformation(i)),
                    Err(j) => Err(j),
                }
            },
            // Bearer Flags
            99 => {
                match PdnType::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::PdnType(i)),
                    Err(j) => Err(j),
                }
            },
            // Procedure Transaction ID
            // MM Context (GSM Keys and Triplets)
            // MM Context (UMTS Keys, Used Chiper, and Quintuplets)
            // MM Context (GSM Keys, Used Chiper, and Quintuplets)
            // MM Context (UMTS Keys and Quintuplets)
            // MM Context (EPS Security Context, Quadruplets and Quintuplets)
            // MM Context (UMTS Key, Quadruplets and Quintuplets)
            // PDN Connection
            // PDU Numbers
            111 => {
                match Ptmsi::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::Ptmsi(i)),
                    Err(j) => Err(j),
                }
            },
            112 => {
                match PtmsiSignature::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::PtmsiSignature(i)),
                    Err(j) => Err(j),
                }
            },
            113 => {
                match HopCounter::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::HopCounter(i)),
                    Err(j) => Err(j),
                }
            },
            114 => {
                match UeTimeZone::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::UeTimeZone(i)),
                    Err(j) => Err(j),
                }
            },
            115 => {
                match TraceReference::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::TraceReference(i)),
                    Err(j) => Err(j),
                }
            },
            // Complete Request Message
            // GUTI
            // F-Container
            // F-Cause
            120 => {
                match PlmnId::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::PlmnId(i)),
                    Err(j) => Err(j),
                }
            },
            // Target Identification
            123 => {
                match PacketFlowId::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::PacketFlowId(i)),
                    Err(j) => Err(j),
                }
            },
            // RAB Context
            // Source RNC PDCP Context Info
            126 => {
                match PortNumber::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::PortNumber(i)),
                    Err(j) => Err(j),
                }
            },
            127 => {
                match ApnRestriction::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::ApnRestriction(i)),
                    Err(j) => Err(j),
                }
            },
            128 => {
                match SelectionMode::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::SelectionMode(i)),
                    Err(j) => Err(j),
                }
            },
            // Source Identification
            131 => {
                match ChangeReportingAction::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::ChangeReportingAction(i)),
                    Err(j) => Err(j),
                }
            },
            // FQ-CSID
            // Channel Needed
            // eMLPP Priority
            135 => {
                match NodeType::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::NodeType(i)),
                    Err(j) => Err(j),
                }
            },
            // FQDN
            // Transaction Identifier
            // MBMS Session Duration
            // MBMS Service Area
            // MBMS Session Identifier
            // MBMS Flow Identifier
            // MBMS IP Multicast Distribution
            // MBMS Distribution Acknowledge
            // RFSP Index
            145 => {
                match Uci::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::Uci(i)),
                    Err(j) => Err(j),
                }
            },
            146 => {
                match CSGInformationReportingAction::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::CSGInformationReportingAction(i)),
                    Err(j) => Err(j),
                }
            },
            // CSG ID
            // CSG Membership Indication
            // Service Indicator
            // Detach Type
            // Local Distiguished Name
            // Node Features
            // MBMS Time to Data Transfer
            // Throttling
            // Allocation/Retention Priority
            // EPC Timer
            // Signalling Priority Indication
            // Temporary Mobile Group Identity
            // Additional MM context SRVCC
            // Additional flags SRVCC
            // MDT Configuration
            // Additional PCO
            // Absolute Time of MBMS Data Transfer
            // H(e)NB Information Reporting
            // IPv4 Configuration Parameters (IP4CP)
            // Change to Report Flags
            // Action Indication
            // TWAN Identifier
            // ULI Timestamp
            // MBMS Flags
            // RAN/NAS Cause
            // CN Operator Selection Entity
            // Trusted WLAN Mode Indication
            // Node Number
            // Node Identifier
            // Presence Reporting Area Action
            // Presence Reporting Area Information
            // TWAN Identifier Timestamp
            // Overload Control Information
            // Load Control Information
            // Metric
            // Sequence Number
            // APN and Relative Capacity
            // WLAN Offloadability Indication
            // Paging and Service Information
            // Integer Number
            // Millisecond Time Stamp
            // Monitoring Event Information
            // ECGI List
            // Remote UE Context
            // Remote User ID
            // Remote UE IP Information
            // CIoT Optimization Support Indication
            // SCEF PDN Connection
            // Header Compression Configuration
            // Extended PCO
            // Serving PLMN Rate Control
            // Counter
            // Mapped UE Usage Type
            // Secondary UE Usage Type
            // UP Function Selection Indication Flags
            // Max Packet Loss Rate
            // APN Rate Control Status
            // Extended Trace Information
            // Monitoring Event Extension Information
            // Special IE type for IE Type Extension
            255 => {
                match PrivateExtension::unmarshal(buffer) {
                    Ok(i) => Ok(InformationElement::PrivateExtension(i)),
                    Err(j) => Err(j),
                }
            },
            _ => Err(GTPV2Error::IEIncorrect(buffer[0])),
        }
    }
/* 
    pub fn get_ie (&self) -> Box<dyn IEs> {
        match *self {
            InformationElement::Imsi(i) => Box::new(i),
            InformationElement::Cause(i) => Box::new(i),
            InformationElement::Recovery(i) => Box::new(i),
            // STN-SR
            InformationElement::Apn(i) => Box::new(i),
            InformationElement::ApnAmbr(i) => Box::new(i),
            InformationElement::Ebi(i) => Box::new(i),
            InformationElement::IpAddress(i) => Box::new(i),
            InformationElement::Mei(i) => Box::new(i),
            InformationElement::Msisdn(i) => Box::new(i),
            InformationElement::Indication(i) => Box::new(i),
            InformationElement::Pco(i) => Box::new(i),
            InformationElement::PdnAddressAllocation(i) => Box::new(i),
            InformationElement::BearerQos(i) => Box::new(i),
            InformationElement::FlowQos(i) => Box::new(i),
            InformationElement::RatType(i) => Box::new(i),
            InformationElement::ServingNetwork(i) => Box::new(i),
            InformationElement::BearerTft(i) => Box::new(i),
            InformationElement::TrafficAggregateDescription(i) => Box::new(i), 
            InformationElement::Uli(i) => Box::new(i),
            InformationElement::Fteid(i) => Box::new(i),
            // Tmsi(Tmsi),
            // Global CN-id
            // S103 PDN Data Forwarding Info
            // S1-U Data Forwarding Info
            // Delay Value
            // Bearer Context
            InformationElement::ChargingId(i) => Box::new(i),
            InformationElement::ChargingCharacteristics(i) => Box::new(i),
            InformationElement::TraceInformation(i) => Box::new(i),
            // Bearer Flags
            InformationElement::PdnType(i) => Box::new(i),
            // Procedure Transaction ID
            // MM Context (GSM Keys and Triplets)
            // MM Context (UMTS Keys, Used Chiper, and Quintuplets)
            // MM Context (GSM Keys, Used Chiper, and Quintuplets)
            // MM Context (UMTS Keys and Quintuplets)
            // MM Context (EPS Security Context, Quadruplets and Quintuplets)
            // MM Context (UMTS Key, Quadruplets and Quintuplets)
            // PDN Connection
            // PDU Numbers
            InformationElement::Ptmsi(i) => Box::new(i),
            InformationElement::PtmsiSignature(i) => Box::new(i),
            InformationElement::HopCounter(i) => Box::new(i),
            InformationElement::UeTimeZone(i) => Box::new(i),
            InformationElement::TraceReference(i) => Box::new(i),
            // Complete Request Message
            // GUTI
            // F-Container
            // F-Cause
            InformationElement::PlmnId(i) => Box::new(i),
            // Target Identification
            InformationElement::PacketFlowId(i) => Box::new(i),
            // RAB Context
            // Source RNC PDCP Context Info
            InformationElement::PortNumber(i) => Box::new(i),
            InformationElement::ApnRestriction(i) => Box::new(i),
            InformationElement::SelectionMode(i) => Box::new(i),
            // Source Identification
            InformationElement::ChangeReportingAction(i) => Box::new(i),
            // FQ-CSID
            // Channel Needed
            // eMLPP Priority
            InformationElement::NodeType(i) => Box::new(i),
            // FQDN
            // Transaction Identifier
            // MBMS Session Duration
            // MBMS Service Area
            // MBMS Session Identifier
            // MBMS Flow Identifier
            // MBMS IP Multicast Distribution
            // MBMS Distribution Acknowledge
            // RFSP Index
            InformationElement::Uci(i) => Box::new(i),
            InformationElement::CSGInformationReportingAction(i) => Box::new(i),
            // CSG ID
            // CSG Membership Indication
            // Service Indicator
            // Detach Type
            // Local Distiguished Name
            InformationElement::NodeFeatures(i) => Box::new(i),
            // MBMS Time to Data Transfer
            // Throttling
            // Allocation/Retention Priority
            // EPC Timer
            // Signalling Priority Indication
            // Temporary Mobile Group Identity
            // Additional MM context SRVCC
            // Additional flags SRVCC
            // MDT Configuration
            // Additional PCO
            // Absolute Time of MBMS Data Transfer
            // H(e)NB Information Reporting
            // IPv4 Configuration Parameters (IP4CP)
            // Change to Report Flags
            // Action Indication
            // TWAN Identifier
            // ULI Timestamp
            // MBMS Flags
            // RAN/NAS Cause
            // CN Operator Selection Entity
            // Trusted WLAN Mode Indication
            // Node Number
            // Node Identifier
            // Presence Reporting Area Action
            // Presence Reporting Area Information
            // TWAN Identifier Timestamp
            // Overload Control Information
            // Load Control Information
            // Metric
            // Sequence Number
            // APN and Relative Capacity
            // WLAN Offloadability Indication
            // Paging and Service Information
            // Integer Number
            // Millisecond Time Stamp
            // Monitoring Event Information
            // ECGI List
            // Remote UE Context
            // Remote User ID
            // Remote UE IP Information
            // CIoT Optimization Support Indication
            // SCEF PDN Connection
            // Header Compression Configuration
            // Extended PCO
            // Serving PLMN Rate Control
            // Counter
            // Mapped UE Usage Type
            // Secondary UE Usage Type
            // UP Function Selection Indication Flags
            // Max Packet Loss Rate
            // APN Rate Control Status
            // Extended Trace Information
            // Monitoring Event Extension Information
            // Special IE type for IE Type Extension
            InformationElement::PrivateExtension(i) => Box::new(i),  
        }
    } */
}

