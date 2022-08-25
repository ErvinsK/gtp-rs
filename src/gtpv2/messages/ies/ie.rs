use crate::gtpv2::{errors::GTPV2Error, messages::ies::*};

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
    BearerContext(GroupedIe),
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
    PdnConnection(GroupedIe),
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
    OverloadControlInfo(GroupedIe),
    LoadControlInfo(GroupedIe),
    // Metric
    // Sequence Number
    // APN and Relative Capacity
    // WLAN Offloadability Indication
    // Paging and Service Information
    // Integer Number
    // Millisecond Time Stamp
    // Monitoring Event Information
    // ECGI List
    RemoteUeContext(GroupedIe),
    // Remote User ID
    // Remote UE IP Information
    // CIoT Optimization Support Indication
    ScefPdnConnection(GroupedIe),
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
    Unknown(Unknown),
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
            InformationElement::BearerContext(i) => i.marshal(buffer),
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
            InformationElement::PdnConnection(i) => i.marshal(buffer),
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
            InformationElement::OverloadControlInfo(i) => i.marshal(buffer),
            InformationElement::LoadControlInfo(i) => i.marshal(buffer),
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
            InformationElement::RemoteUeContext(i) => i.marshal(buffer),
            // Remote User ID
            // Remote UE IP Information
            // CIoT Optimization Support Indication
            InformationElement::ScefPdnConnection(i) => i.marshal(buffer),
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
            InformationElement::Unknown(i) => i.marshal(buffer),
        }
    }

    pub fn encoder(message:Vec<InformationElement>, buffer: &mut Vec<u8>) {
        for i in message.into_iter() {
            i.marshal(buffer);
        }
    }

    pub fn decoder(buffer:&[u8]) -> Result<Vec<InformationElement>, GTPV2Error> {
        let mut ies:Vec<InformationElement>=vec!();
        let mut cursor:usize = 0;
            loop {
                if cursor >= buffer.len() {
                    break;
                }
                match buffer[cursor] {
                    1 => {
                        match Imsi::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::Imsi(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    2 => {
                        match Cause::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::Cause(i));
                            },
                            Err(j) => return Err(j),
                        }
                    }, 
                    3 => {
                        match Recovery::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::Recovery(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    // STN-SR   
                    71 => {
                        match Apn::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::Apn(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    72 => {
                        match ApnAmbr::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::ApnAmbr(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    73 => {
                        match Ebi::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::Ebi(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    74 => {
                        match IpAddress::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::IpAddress(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    75 => {
                        match Mei::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::Mei(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    76 => {
                        match Msisdn::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::Msisdn(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    77 => {
                        match Indication::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::Indication(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    78 => {
                        match Pco::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::Pco(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    79 => {
                        match PdnAddressAllocation::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::PdnAddressAllocation(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    80 => {
                        match BearerQos::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::BearerQos(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    81 => {
                        match FlowQos::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::FlowQos(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    82 => {
                        match RatType::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::RatType(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    83 => {
                        match ServingNetwork::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::ServingNetwork(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    84 => {
                        match BearerTft::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::BearerTft(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    85 => {
                        match TrafficAggregateDescription::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::TrafficAggregateDescription(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    86 => {
                        match Uli::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::Uli(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    87 => {
                        match Fteid::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::Fteid(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    // Tmsi(Tmsi),
                    // Global CN-id
                    // S103 PDN Data Forwarding Info
                    // S1-U Data Forwarding Info
                    // Delay Value
                    93 => {
                        match GroupedIe::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::BearerContext(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    94 => {
                        match ChargingId::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::ChargingId(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    95 => {
                        match ChargingCharacteristics::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::ChargingCharacteristics(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    96 => {
                        match TraceInformation::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::TraceInformation(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    // Bearer Flags
                    99 => {
                        match PdnType::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::PdnType(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    // Procedure Transaction ID
                    // MM Context (GSM Keys and Triplets)
                    // MM Context (UMTS Keys, Used Chiper, and Quintuplets)
                    // MM Context (GSM Keys, Used Chiper, and Quintuplets)
                    // MM Context (UMTS Keys and Quintuplets)
                    // MM Context (EPS Security Context, Quadruplets and Quintuplets)
                    // MM Context (UMTS Key, Quadruplets and Quintuplets)
                    109 => {
                        match GroupedIe::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::PdnConnection(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    // PDN Connection
                    // PDU Numbers
                    111 => {
                        match Ptmsi::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::Ptmsi(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    112 => {
                        match PtmsiSignature::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::PtmsiSignature(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    113 => {
                        match HopCounter::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::HopCounter(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    114 => {
                        match UeTimeZone::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::UeTimeZone(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    115 => {
                        match TraceReference::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::TraceReference(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    // Complete Request Message
                    // GUTI
                    // F-Container
                    // F-Cause
                    120 => {
                        match PlmnId::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::PlmnId(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    // Target Identification
                    123 => {
                        match PacketFlowId::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::PacketFlowId(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    // RAB Context
                    // Source RNC PDCP Context Info
                    126 => {
                        match PortNumber::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::PortNumber(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    127 => {
                        match ApnRestriction::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::ApnRestriction(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    128 => {
                        match SelectionMode::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::SelectionMode(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    // Source Identification
                    131 => {
                        match ChangeReportingAction::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::ChangeReportingAction(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    // FQ-CSID
                    // Channel Needed
                    // eMLPP Priority
                    135 => {
                        match NodeType::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::NodeType(i));
                            },
                            Err(j) => return Err(j),
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
                        match Uci::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::Uci(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    146 => {
                        match CSGInformationReportingAction::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::CSGInformationReportingAction(i));
                            },
                            Err(j) => return Err(j),
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
                    180 => {
                        match GroupedIe::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::BearerContext(i));
                            },
                            Err(j) => return Err(j),
                        }
                    }, 
                    181 => {
                        match GroupedIe::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::BearerContext(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
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
                    195 => {
                        match GroupedIe::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::ScefPdnConnection(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
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
                        match PrivateExtension::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::PrivateExtension(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                    _ => {
                        match Unknown::unmarshal(&buffer[cursor..]) {
                            Ok(i) => {
                                cursor+=i.len();
                                ies.push(InformationElement::Unknown(i));
                            },
                            Err(j) => return Err(j),
                        }
                    },
                }
            }
            Ok(ies)
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

impl From<Cause> for InformationElement {
    fn from(i: Cause) -> Self {
        InformationElement::Cause(i)
    }
}