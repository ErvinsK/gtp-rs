/*

List of GTPv2-C IEs according to 3GPP TS 29.274 V18.0.0 (2022-09)

IE Type         Name                                                            Status
0               Reserved                                                        NA
1               International Mobile Subscriber Identity (IMSI)                 Implemented
2               Cause                                                           Implemented    
3               Recovery (Restart Counter)                                      Implemented
4 to 34         Reserved for S101 interface
35 to 50        Reserved for S121 interface
51              STN-SR
56              SRVCC Cause
52 to 70        Reserved for Sv interface
71              Access Point Name (APN)                                         Implemented
72              Aggregate Maximum Bit Rate (AMBR)                               Implemented
73              EPS Bearer ID (EBI)
74              IP Address
75              Mobile Equipment Identity (MEI)
76              MSISDN
77              Indication
78              Protocol Configuration Options (PCO)
79              PDN Address Allocation (PAA)
80              Bearer Level Quality of Service (Bearer QoS)
81              Flow Quality of Service (Flow QoS)
82              RAT Type
83              Serving Network
84              EPS Bearer Level Traffic Flow Template (Bearer TFT)
85              Traffic Aggregation Description (TAD)
86              User Location Information (ULI)
87              Fully Qualified Tunnel Endpoint Identifier (F-TEID)
88              TMSI
89              Global CN-Id
90              S103 PDN Data Forwarding Info (S103PDF)
91              S1-U Data Forwarding Info (S1UDF)
92              Delay Value
93              Bearer Context
94              Charging ID
95              Charging Characteristics
96              Trace Information
97              Bearer Flags
98              Reserved
99              PDN Type
100             Procedure Transaction ID
101             Reserved
102             Reserved
103             MM Context (GSM Key and Triplets)
104             MM Context (UMTS Key, Used Cipher and Quintuplets)
105             MM Context (GSM Key, Used Cipher and Quintuplets)
106             MM Context (UMTS Key and Quintuplets)
107             MM Context (EPS Security Context, Quadruplets and Quintuplets)
108             MM Context (UMTS Key, Quadruplets and Quintuplets)
109             PDN Connection
110             PDU Numbers
111             P-TMSI
112             P-TMSI Signature
113             Hop Counter
114             UE Time Zone
115             Trace Reference
116             Complete Request Message
117             GUTI
118             F-Container
119             F-Cause
120             PLMN ID
121             Target Identification
122             Reserved
123             Packet Flow ID
124             RAB Context
125             Source RNC PDCP Context Info
126             Port Number
127             APN Restriction
128             Selection Mode
129             Source Identification
130             Reserved
131             Change Reporting Action
132             Fully Qualified PDN Connection Set Identifier (FQ-CSID)
133             Channel needed
134             eMLPP Priority
135             Node Type
136             Fully Qualified Domain Name (FQDN)
137             Transaction Identifier (TI)
138             MBMS Session Duration
139             MBMS Service Area
140             MBMS Session Identifier
141             MBMS Flow Identifier
142             MBMS IP Multicast Distribution
143             MBMS Distribution Acknowledge
144             RFSP Index
145             User CSG Information (UCI)
146             CSG Information Reporting Action
147             CSG ID
148             CSG Membership Indication (CMI)
149             Service indicator
150             Detach Type
151             Local Distiguished Name (LDN)
152             Node Features
153             MBMS Time to Data Transfer
154             Throttling
155             Allocation/Retention Priority (ARP)
156             EPC Timer
157             Signalling Priority Indication
158             Temporary Mobile Group Identity (TMGI)
159             Additional MM context for SRVCC
160             Additional flags for SRVCC
161             Reserved
162             MDT Configuration
163             Additional Protocol Configuration Options (APCO)
164             Absolute Time of MBMS Data Transfer
165             H(e)NB Information Reporting
166             IPv4 Configuration Parameters (IP4CP)
167             Change to Report Flags
168             Action Indication
169             TWAN Identifier
170             ULI Timestamp
171             MBMS Flags
172             RAN/NAS Cause
173             CN Operator Selection Entity
174             Trusted WLAN Mode Indication
175             Node Number
176             Node Identifier
177             Presence Reporting Area Action
178             Presence Reporting Area Information
179             TWAN Identifier Timestamp
180             Overload Control Information
181             Load Control Information
182             Metric
183             Sequence Number
184             APN and Relative Capacity
185             WLAN Offloadability Indication
186             Paging and Service Information
187             Integer Number
188             Millisecond Time Stamp
189             Monitoring Event Information
190             ECGI List
191             Remote UE Context
192             Remote User ID
193             Remote UE IP information
194             CIoT Optimizations Support Indication
195             SCEF PDN Connection
196             Header Compression Configuration
197             Extended Protocol Configuration Options (ePCO)
198             Serving PLMN Rate Control
199             Counter
200             Mapped UE Usage Type
201             Secondary RAT Usage Data Report
202             UP Function Selection Indication Flags
203             Maximum Packet Loss Rate
204             APN Rate Control Status
205             Extended Trace Information
206             Monitoring Event Extension Information
207             Additional RRM Policy Index
208             V2X Context
209             PC5 QoS Parameters
210             Services Authorized
211             Bit Rate
212             PC5 QoS Flow
213             SGi PtP Tunnel Address
214             PGW Change Info
215             PGW FQDN
216             Group Id
217             PSCell ID
218             UP Security Policy
219             Alternative IMSI
220 to 253      Spare. For future use.
254             Special IE type for IE Type Extension
255             Private Extension
256 to 65535    Spare. For future use.



*/



use crate::gtpv2::{errors::GTPV2Error, messages::ies::*};

#[derive(Debug, Clone, PartialEq)]
pub enum InformationElement {
    Imsi(Imsi),
    Cause(Cause),
    Recovery(Recovery),
    StnSr(StnSr),
    Apn(Apn),
    ApnAmbr(Ambr),
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
    Tmsi(Tmsi),
    GlobalCnId(GlobalCnId),
    S103pdf(S103pdf),
    S1udf(S1udf),
    DelayValue(DelayValue),
    BearerContext(BearerContext),
    ChargingId(ChargingId),
    ChargingCharacteristics(ChargingCharacteristics),
    TraceInformation(TraceInformation),
    BearerFlags(BearerFlags),
    PdnType(PdnType),
    Pti(Pti),
    MmContextGsmKeyTriplets(MmContextGsmKeyTriplets),
    MmContextUmtsKeyCipherQuintuplets(MmContextUmtsKeyCipherQuintuplets),
    MmContextGsmKeyCipherQuintuplets(MmContextGsmKeyCipherQuintuplets),
    MmContextUmtsKeyQuintuplets(MmContextUmtsKeyQuintuplets),
    MmContextEpsSecurityContextQuadruplets(MmContextEpsSecurityContextQuadruplets),
    MmContextUmtsKeyQuadrupletsQuintuplets(MmContextUmtsKeyQuadrupletsQuintuplets),
    PdnConnections(PdnConnections),
    PduNumbers(PduNumbers),
    Ptmsi(Ptmsi),
    PtmsiSignature(PtmsiSignature),
    HopCounter(HopCounter),
    UeTimeZone(UeTimeZone),
    TraceReference(TraceReference),
    CompleteRequestMessage(CompleteRequestMessage),
    Guti(Guti),
    Fcontainer(Fcontainer),
    Fcause(Fcause),
    PlmnId(PlmnId),
    TargetIdentification(TargetIdentification),
    PacketFlowId(PacketFlowId),
    RabContext(RabContext),
    SrcRncPdcpCtxInfo(SourceRncPdcpContextInfo),
    PortNumber(PortNumber),
    ApnRestriction(ApnRestriction),
    SelectionMode(SelectionMode),
    SourceIdentification(SourceIdentification),
    ChangeReportingAction(ChangeReportingAction),
    Fqcsid(Fqcsid),
    ChannelNeeded(ChannelNeeded),
    EmlppPriority(EmlppPriority),
    NodeType(NodeType),
    Fqdn(Fqdn),
    TransactionIdentifier(TransactionIdentifier),
    MbmsSa(MbmsSa),
    MbmsSd(MbmsSd),
    MbmsSessionId(MbmsSessionId),
    MbmsFlowId(MbmsFlowId),
    MbmsIpMulticastDistribution(MbmsIpMulticastDistribution),
    MbmsDistributionAck(MbmsDistributionAck),
    RfspIndex(RfspIndex),
    Uci(Uci),
    CSGInformationReportingAction(CSGInformationReportingAction),
    CsgId(CsgId),
    CsgMembershipIndication(CsgMembershipIndication),
    ServiceIndicator(ServiceIndicator),
    DetachType(DetachType),
    Ldn(Ldn),
    NodeFeatures(NodeFeatures),
    MbmsTimeToDataTransfer(MbmsTimeToDataTransfer),
    Throttling(Throttling),
    Arp(Arp),
    EpcTimer(EpcTimer),
    Spi(Spi),
    Tmgi(Tmgi),
    AdditionalMmContextForSrvcc(AdditionalMmContextForSrvcc),
    AdditionalFlagsSrvcc(AdditionalFlagsSrvcc),
    MdtConfiguration(MdtConfiguration),
    Apco(Apco),
    AbsoluteTimeMbmsDataTransfer(AbsoluteTimeMbmsDataTransfer),
    HenbInfoReporting(HenbInfoReporting),
    Ip4Cp(Ip4Cp),
    ChangeToReportFlags(ChangeToReportFlags),
    ActionIndication(ActionIndication),
    TwanId(TwanId),
    UliTimestamp(UliTimestamp),
    MbmsFlags(MbmsFlags),
    RanNasCause(RanNasCause),
    CnOperatorSelectionEntity(CnOperatorSelectionEntity),
    Twmi(Twmi),
    NodeNumber(NodeNumber),
    NodeIdentifier(NodeIdentifier),
    PresenceReportingAreaAction(PresenceReportingAreaAction),
    PresenceReportingAreaInformation(PresenceReportingAreaInformation),
    TwanIdTimeStamp(TwanIdTimeStamp),
    OverloadControlInfo(OverloadControlInfo),
    LoadControlInfo(LoadControl),
    Metric(Metric),
    Sqn(Sqn),
    ApnRelativeCapacity(ApnRelativeCapacity),
    WlanOffloadIndication(WlanOffloadIndication),
    PagingServiceInfo(PagingServiceInfo),
    IntegerNumber(IntegerNumber),
    MilliSecondTimeStamp(MilliSecondTimeStamp),
    MonitoringEventInformation(MonitoringEventInformation),
    EcgiList(EcgiList),
    RemoteUeContext(RemoteUeContext),
    RemoteUserId(RemoteUserId),
    RemoteUeIpInformation(RemoteUeIpInformation),
    CIoTOptimizationSupportIndication(CIoTOptimizationSupportIndication),
    ScefPdnConnections(ScefPdnConnections),
    HeaderCompressionConfiguration(HeaderCompressionConfiguration),
    Epco(Epco),
    ServingPlmnRateControl(ServingPlmnRateControl),
    Counter(Counter),
    MappedUeUsageType(MappedUeUsageType),
    SecondaryRatUsageDataReport(SecondaryRatUsageDataReport),
    UpFunctionSelectionIndicationFlags(UpFunctionSelectionIndicationFlags),
    MaxPacketLossRate(MaxPacketLossRate),
    ApnRateControlStatus(ApnRateControlStatus),
    ExtendedTraceInformation(ExtendedTraceInformation),
    MonitoringEventExtensionInfo(MonitoringEventExtensionInfo),
    SpecialIEWithTypeExt(SpecialIEWithTypeExt),
    PrivateExtension(PrivateExtension),
    Unknown(Unknown),
}

impl InformationElement {
    pub fn marshal(self, buffer: &mut Vec<u8>) {
        match self {
            InformationElement::Imsi(i) => i.marshal(buffer),
            InformationElement::Cause(i) => i.marshal(buffer),
            InformationElement::Recovery(i) => i.marshal(buffer),
            InformationElement::StnSr(i) => i.marshal(buffer),
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
            InformationElement::Tmsi(i) => i.marshal(buffer),
            InformationElement::GlobalCnId(i) => i.marshal(buffer),
            InformationElement::S103pdf(i) => i.marshal(buffer),
            InformationElement::S1udf(i) => i.marshal(buffer),
            InformationElement::DelayValue(i) => i.marshal(buffer),
            InformationElement::BearerContext(i) => i.marshal(buffer),
            InformationElement::ChargingId(i) => i.marshal(buffer),
            InformationElement::ChargingCharacteristics(i) => i.marshal(buffer),
            InformationElement::TraceInformation(i) => i.marshal(buffer),
            InformationElement::BearerFlags(i) => i.marshal(buffer),
            InformationElement::PdnType(i) => i.marshal(buffer),
            InformationElement::Pti(i) => i.marshal(buffer),
            InformationElement::MmContextGsmKeyTriplets(i) => i.marshal(buffer),
            InformationElement::MmContextUmtsKeyCipherQuintuplets(i) => i.marshal(buffer),
            InformationElement::MmContextGsmKeyCipherQuintuplets(i) => i.marshal(buffer),
            InformationElement::MmContextUmtsKeyQuintuplets(i) => i.marshal(buffer),
            InformationElement::MmContextEpsSecurityContextQuadruplets(i) => i.marshal(buffer),
            InformationElement::MmContextUmtsKeyQuadrupletsQuintuplets(i) => i.marshal(buffer),
            InformationElement::PdnConnections(i) => i.marshal(buffer),
            InformationElement::PduNumbers(i) => i.marshal(buffer),
            InformationElement::Ptmsi(i) => i.marshal(buffer),
            InformationElement::PtmsiSignature(i) => i.marshal(buffer),
            InformationElement::HopCounter(i) => i.marshal(buffer),
            InformationElement::UeTimeZone(i) => i.marshal(buffer),
            InformationElement::TraceReference(i) => i.marshal(buffer),
            InformationElement::CompleteRequestMessage(i) => i.marshal(buffer),
            InformationElement::Guti(i) => i.marshal(buffer),
            InformationElement::Fcontainer(i) => i.marshal(buffer),
            InformationElement::Fcause(i) => i.marshal(buffer),
            InformationElement::PlmnId(i) => i.marshal(buffer),
            InformationElement::TargetIdentification(i) => i.marshal(buffer),
            InformationElement::PacketFlowId(i) => i.marshal(buffer),
            InformationElement::RabContext(i) => i.marshal(buffer),
            InformationElement::SrcRncPdcpCtxInfo(i) => i.marshal(buffer),
            InformationElement::PortNumber(i) => i.marshal(buffer),
            InformationElement::ApnRestriction(i) => i.marshal(buffer),
            InformationElement::SelectionMode(i) => i.marshal(buffer),
            InformationElement::SourceIdentification(i) => i.marshal(buffer),
            InformationElement::ChangeReportingAction(i) => i.marshal(buffer),
            InformationElement::Fqcsid(i) => i.marshal(buffer),
            InformationElement::ChannelNeeded(i) => i.marshal(buffer),
            InformationElement::EmlppPriority(i) => i.marshal(buffer),
            InformationElement::NodeType(i) => i.marshal(buffer),
            InformationElement::Fqdn(i) => i.marshal(buffer),
            InformationElement::TransactionIdentifier(i) => i.marshal(buffer),
            InformationElement::MbmsSa(i) => i.marshal(buffer),
            InformationElement::MbmsSd(i) => i.marshal(buffer),
            InformationElement::MbmsSessionId(i) => i.marshal(buffer),
            InformationElement::MbmsFlowId(i) => i.marshal(buffer),
            InformationElement::MbmsIpMulticastDistribution(i) => i.marshal(buffer),
            InformationElement::MbmsDistributionAck(i) => i.marshal(buffer),
            InformationElement::RfspIndex(i) => i.marshal(buffer),
            InformationElement::Uci(i) => i.marshal(buffer),
            InformationElement::CSGInformationReportingAction(i) => i.marshal(buffer),
            InformationElement::CsgId(i) => i.marshal(buffer),
            InformationElement::CsgMembershipIndication(i) => i.marshal(buffer),
            InformationElement::ServiceIndicator(i) => i.marshal(buffer),
            InformationElement::DetachType(i) => i.marshal(buffer),
            InformationElement::Ldn(i) => i.marshal(buffer),
            InformationElement::NodeFeatures(i) => i.marshal(buffer),
            InformationElement::MbmsTimeToDataTransfer(i) => i.marshal(buffer),
            InformationElement::Throttling(i) => i.marshal(buffer),
            InformationElement::Arp(i) => i.marshal(buffer),
            InformationElement::EpcTimer(i) => i.marshal(buffer),
            InformationElement::Spi(i) => i.marshal(buffer),
            InformationElement::Tmgi(i) => i.marshal(buffer),
            InformationElement::AdditionalMmContextForSrvcc(i) => i.marshal(buffer),
            InformationElement::AdditionalFlagsSrvcc(i) => i.marshal(buffer),
            InformationElement::MdtConfiguration(i) => i.marshal(buffer),
            InformationElement::Apco(i) => i.marshal(buffer),
            InformationElement::AbsoluteTimeMbmsDataTransfer(i) => i.marshal(buffer),
            InformationElement::HenbInfoReporting(i) => i.marshal(buffer),
            InformationElement::Ip4Cp(i) => i.marshal(buffer),
            InformationElement::ChangeToReportFlags(i) => i.marshal(buffer),
            InformationElement::ActionIndication(i) => i.marshal(buffer),
            InformationElement::TwanId(i) => i.marshal(buffer),
            InformationElement::UliTimestamp(i) => i.marshal(buffer),
            InformationElement::MbmsFlags(i) => i.marshal(buffer),
            InformationElement::RanNasCause(i) => i.marshal(buffer),
            InformationElement::CnOperatorSelectionEntity(i) => i.marshal(buffer),
            InformationElement::Twmi(i) => i.marshal(buffer),
            InformationElement::NodeNumber(i) => i.marshal(buffer),
            InformationElement::NodeIdentifier(i) => i.marshal(buffer),
            InformationElement::PresenceReportingAreaAction(i) => i.marshal(buffer),
            InformationElement::PresenceReportingAreaInformation(i) => i.marshal(buffer),
            InformationElement::TwanIdTimeStamp(i) => i.marshal(buffer),
            InformationElement::OverloadControlInfo(i) => i.marshal(buffer),
            InformationElement::LoadControlInfo(i) => i.marshal(buffer),
            InformationElement::Metric(i) => i.marshal(buffer),
            InformationElement::Sqn(i) => i.marshal(buffer),
            InformationElement::ApnRelativeCapacity(i) => i.marshal(buffer),
            InformationElement::WlanOffloadIndication(i) => i.marshal(buffer),
            InformationElement::PagingServiceInfo(i) => i.marshal(buffer),
            InformationElement::IntegerNumber(i) => i.marshal(buffer),
            InformationElement::MilliSecondTimeStamp(i) => i.marshal(buffer),
            InformationElement::MonitoringEventInformation(i) => i.marshal(buffer),
            InformationElement::EcgiList(i) => i.marshal(buffer),
            InformationElement::RemoteUeContext(i) => i.marshal(buffer),
            InformationElement::RemoteUserId(i) => i.marshal(buffer),
            InformationElement::RemoteUeIpInformation(i) => i.marshal(buffer),
            InformationElement::CIoTOptimizationSupportIndication(i) => i.marshal(buffer),
            InformationElement::ScefPdnConnections(i) => i.marshal(buffer),
            InformationElement::HeaderCompressionConfiguration(i) => i.marshal(buffer),
            InformationElement::Epco(i) => i.marshal(buffer),
            InformationElement::ServingPlmnRateControl(i) => i.marshal(buffer),
            InformationElement::Counter(i) => i.marshal(buffer),
            InformationElement::MappedUeUsageType(i) => i.marshal(buffer),
            InformationElement::SecondaryRatUsageDataReport(i) => i.marshal(buffer),
            InformationElement::UpFunctionSelectionIndicationFlags(i) => i.marshal(buffer),
            InformationElement::MaxPacketLossRate(i) => i.marshal(buffer),
            InformationElement::ApnRateControlStatus(i) => i.marshal(buffer),
            InformationElement::ExtendedTraceInformation(i) => i.marshal(buffer),
            InformationElement::MonitoringEventExtensionInfo(i) => i.marshal(buffer),
            InformationElement::SpecialIEWithTypeExt(i) => i.marshal(buffer),
            InformationElement::PrivateExtension(i) => i.marshal(buffer),
            InformationElement::Unknown(i) => i.marshal(buffer),
        }
    }

    pub fn encoder(message: Vec<InformationElement>, buffer: &mut Vec<u8>) {
        for i in message.into_iter() {
            i.marshal(buffer);
        }
    }

    pub fn decoder(buffer: &[u8]) -> Result<Vec<InformationElement>, GTPV2Error> {
        let mut ies: Vec<InformationElement> = vec![];
        let mut cursor: usize = 0;
        loop {
            if cursor >= buffer.len() {
                break;
            }
            match buffer[cursor] {
                1 => match Imsi::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::Imsi(i));
                    }
                    Err(j) => return Err(j),
                },
                2 => match Cause::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::Cause(i));
                    }
                    Err(j) => return Err(j),
                },
                3 => match Recovery::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::Recovery(i));
                    }
                    Err(j) => return Err(j),
                },
                51 => match StnSr::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::StnSr(i));
                    }
                    Err(j) => return Err(j),
                },
                71 => match Apn::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::Apn(i));
                    }
                    Err(j) => return Err(j),
                },
                72 => match Ambr::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::ApnAmbr(i));
                    }
                    Err(j) => return Err(j),
                },
                73 => match Ebi::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::Ebi(i));
                    }
                    Err(j) => return Err(j),
                },
                74 => match IpAddress::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::IpAddress(i));
                    }
                    Err(j) => return Err(j),
                },
                75 => match Mei::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::Mei(i));
                    }
                    Err(j) => return Err(j),
                },
                76 => match Msisdn::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::Msisdn(i));
                    }
                    Err(j) => return Err(j),
                },
                77 => match Indication::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::Indication(i));
                    }
                    Err(j) => return Err(j),
                },
                78 => match Pco::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::Pco(i));
                    }
                    Err(j) => return Err(j),
                },
                79 => match PdnAddressAllocation::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::PdnAddressAllocation(i));
                    }
                    Err(j) => return Err(j),
                },
                80 => match BearerQos::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::BearerQos(i));
                    }
                    Err(j) => return Err(j),
                },
                81 => match FlowQos::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::FlowQos(i));
                    }
                    Err(j) => return Err(j),
                },
                82 => match RatType::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::RatType(i));
                    }
                    Err(j) => return Err(j),
                },
                83 => match ServingNetwork::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::ServingNetwork(i));
                    }
                    Err(j) => return Err(j),
                },
                84 => match BearerTft::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::BearerTft(i));
                    }
                    Err(j) => return Err(j),
                },
                85 => match TrafficAggregateDescription::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::TrafficAggregateDescription(i));
                    }
                    Err(j) => return Err(j),
                },
                86 => match Uli::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::Uli(i));
                    }
                    Err(j) => return Err(j),
                },
                87 => match Fteid::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::Fteid(i));
                    }
                    Err(j) => return Err(j),
                },
                88 => match Tmsi::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::Tmsi(i));
                    }
                    Err(j) => return Err(j),
                },
                89 => match GlobalCnId::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::GlobalCnId(i));
                    }
                    Err(j) => return Err(j),
                },
                90 => match S103pdf::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::S103pdf(i));
                    }
                    Err(j) => return Err(j),
                },
                91 => match S1udf::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::S1udf(i));
                    }
                    Err(j) => return Err(j),
                },
                92 => match DelayValue::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::DelayValue(i));
                    }
                    Err(j) => return Err(j),
                },
                93 => match BearerContext::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::BearerContext(i));
                    }
                    Err(j) => return Err(j),
                },
                94 => match ChargingId::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::ChargingId(i));
                    }
                    Err(j) => return Err(j),
                },
                95 => match ChargingCharacteristics::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::ChargingCharacteristics(i));
                    }
                    Err(j) => return Err(j),
                },
                96 => match TraceInformation::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::TraceInformation(i));
                    }
                    Err(j) => return Err(j),
                },
                97 => match BearerFlags::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::BearerFlags(i));
                    }
                    Err(j) => return Err(j),
                },
                99 => match PdnType::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::PdnType(i));
                    }
                    Err(j) => return Err(j),
                },
                100 => match Pti::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::Pti(i));
                    }
                    Err(j) => return Err(j),
                },
                103 => match MmContextGsmKeyTriplets::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::MmContextGsmKeyTriplets(i));
                    }
                    Err(j) => return Err(j),
                },
                104 => match MmContextUmtsKeyCipherQuintuplets::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::MmContextUmtsKeyCipherQuintuplets(i));
                    }
                    Err(j) => return Err(j),
                },
                105 => match MmContextGsmKeyCipherQuintuplets::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::MmContextGsmKeyCipherQuintuplets(i));
                    }
                    Err(j) => return Err(j),
                },
                106 => match MmContextUmtsKeyQuintuplets::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::MmContextUmtsKeyQuintuplets(i));
                    }
                    Err(j) => return Err(j),
                },
                107 => match MmContextEpsSecurityContextQuadruplets::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::MmContextEpsSecurityContextQuadruplets(
                            i,
                        ));
                    }
                    Err(j) => return Err(j),
                },
                108 => match MmContextUmtsKeyQuadrupletsQuintuplets::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::MmContextUmtsKeyQuadrupletsQuintuplets(
                            i,
                        ));
                    }
                    Err(j) => return Err(j),
                },
                109 => match PdnConnections::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::PdnConnections(i));
                    }
                    Err(j) => return Err(j),
                },
                110 => match PduNumbers::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::PduNumbers(i));
                    }
                    Err(j) => return Err(j),
                },
                111 => match Ptmsi::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::Ptmsi(i));
                    }
                    Err(j) => return Err(j),
                },
                112 => match PtmsiSignature::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::PtmsiSignature(i));
                    }
                    Err(j) => return Err(j),
                },
                113 => match HopCounter::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::HopCounter(i));
                    }
                    Err(j) => return Err(j),
                },
                114 => match UeTimeZone::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::UeTimeZone(i));
                    }
                    Err(j) => return Err(j),
                },
                115 => match TraceReference::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                116 => match CompleteRequestMessage::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                117 => match Guti::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                118 => match Fcontainer::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                119 => match Fcause::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                120 => match PlmnId::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                121 => match TargetIdentification::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                123 => match PacketFlowId::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                124 => match RabContext::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                125 => match SourceRncPdcpContextInfo::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                126 => match PortNumber::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                127 => match ApnRestriction::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                128 => match SelectionMode::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                129 => match SourceIdentification::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                131 => match ChangeReportingAction::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                132 => match Fqcsid::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                133 => match ChannelNeeded::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                134 => match EmlppPriority::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                135 => match NodeType::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                136 => match Fqdn::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                137 => match TransactionIdentifier::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                138 => match MbmsSd::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::MbmsSd(i));
                    }
                    Err(j) => return Err(j),
                },
                139 => match MbmsSa::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::MbmsSa(i));
                    }
                    Err(j) => return Err(j),
                },
                140 => match MbmsSessionId::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::MbmsSessionId(i));
                    }
                    Err(j) => return Err(j),
                },
                141 => match MbmsFlowId::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::MbmsFlowId(i));
                    }
                    Err(j) => return Err(j),
                },
                142 => match MbmsIpMulticastDistribution::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::MbmsIpMulticastDistribution(i));
                    }
                    Err(j) => return Err(j),
                },
                143 => match MbmsDistributionAck::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::MbmsDistributionAck(i));
                    }
                    Err(j) => return Err(j),
                },
                144 => match RfspIndex::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                145 => match Uci::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                146 => match CSGInformationReportingAction::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                147 => match CsgId::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                148 => match CsgMembershipIndication::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                149 => match ServiceIndicator::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                150 => match DetachType::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                151 => match Ldn::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                152 => match NodeFeatures::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                153 => match MbmsTimeToDataTransfer::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::MbmsTimeToDataTransfer(i));
                    }
                    Err(j) => return Err(j),
                },
                154 => match Throttling::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                155 => match Arp::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                156 => match EpcTimer::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                157 => match Spi::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                158 => match Tmgi::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                159 => match AdditionalMmContextForSrvcc::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::AdditionalMmContextForSrvcc(i));
                    }
                    Err(j) => return Err(j),
                },
                160 => match AdditionalFlagsSrvcc::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::AdditionalFlagsSrvcc(i));
                    }
                    Err(j) => return Err(j),
                },
                162 => match MdtConfiguration::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::MdtConfiguration(i));
                    }
                    Err(j) => return Err(j),
                },
                163 => match Apco::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::Apco(i));
                    }
                    Err(j) => return Err(j),
                },
                164 => match AbsoluteTimeMbmsDataTransfer::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::AbsoluteTimeMbmsDataTransfer(i));
                    }
                    Err(j) => return Err(j),
                },
                165 => match HenbInfoReporting::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                166 => match Ip4Cp::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                167 => match ChangeToReportFlags::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                168 => match ActionIndication::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                169 => match ActionIndication::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                170 => match UliTimestamp::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                171 => match MbmsFlags::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::MbmsFlags(i));
                    }
                    Err(j) => return Err(j),
                },
                172 => match RanNasCause::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                173 => match CnOperatorSelectionEntity::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                174 => match Twmi::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                175 => match NodeNumber::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                176 => match NodeIdentifier::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                177 => match PresenceReportingAreaAction::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                178 => match PresenceReportingAreaInformation::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                179 => match TwanIdTimeStamp::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                180 => {
                    // Overload Control Information
                    match OverloadControlInfo::unmarshal(&buffer[cursor..]) {
                        Ok(i) => {
                            cursor += i.len();
                            ies.push(InformationElement::OverloadControlInfo(i));
                        }
                        Err(j) => return Err(j),
                    }
                }
                181 => {
                    // Load Control Information
                    match LoadControl::unmarshal(&buffer[cursor..]) {
                        Ok(i) => {
                            cursor += i.len();
                            ies.push(InformationElement::LoadControlInfo(i));
                        }
                        Err(j) => return Err(j),
                    }
                }
                182 => match Metric::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::Metric(i));
                    }
                    Err(j) => return Err(j),
                },
                183 => match Sqn::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::Sqn(i));
                    }
                    Err(j) => return Err(j),
                },
                184 => match ApnRelativeCapacity::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::ApnRelativeCapacity(i));
                    }
                    Err(j) => return Err(j),
                },
                185 => match WlanOffloadIndication::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::WlanOffloadIndication(i));
                    }
                    Err(j) => return Err(j),
                },
                186 => match PagingServiceInfo::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                187 => match IntegerNumber::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::IntegerNumber(i));
                    }
                    Err(j) => return Err(j),
                },
                188 => match MilliSecondTimeStamp::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                189 => match MonitoringEventInformation::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                190 => match EcgiList::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::EcgiList(i));
                    }
                    Err(j) => return Err(j),
                },
                191 => {
                    // Remote UE Context
                    match RemoteUeContext::unmarshal(&buffer[cursor..]) {
                        Ok(i) => {
                            cursor += i.len();
                            ies.push(InformationElement::RemoteUeContext(i));
                        }
                        Err(j) => return Err(j),
                    }
                }
                192 => match RemoteUserId::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::RemoteUserId(i));
                    }
                    Err(j) => return Err(j),
                },
                193 => match RemoteUeIpInformation::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                194 => match CIoTOptimizationSupportIndication::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(i.into());
                    }
                    Err(j) => return Err(j),
                },
                195 => match ScefPdnConnections::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::ScefPdnConnections(i));
                    }
                    Err(j) => return Err(j),
                },
                196 => match HeaderCompressionConfiguration::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::HeaderCompressionConfiguration(i));
                    }
                    Err(j) => return Err(j),
                },
                197 => match Epco::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::Epco(i));
                    }
                    Err(j) => return Err(j),
                },
                198 => match ServingPlmnRateControl::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::ServingPlmnRateControl(i));
                    }
                    Err(j) => return Err(j),
                },
                199 => match Counter::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::Counter(i));
                    }
                    Err(j) => return Err(j),
                },
                200 => match MappedUeUsageType::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::MappedUeUsageType(i));
                    }
                    Err(j) => return Err(j),
                },
                201 => match SecondaryRatUsageDataReport::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::SecondaryRatUsageDataReport(i));
                    }
                    Err(j) => return Err(j),
                },
                202 => match UpFunctionSelectionIndicationFlags::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::UpFunctionSelectionIndicationFlags(i));
                    }
                    Err(j) => return Err(j),
                },
                203 => match MaxPacketLossRate::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::MaxPacketLossRate(i));
                    }
                    Err(j) => return Err(j),
                },
                204 => match ApnRateControlStatus::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::ApnRateControlStatus(i));
                    }
                    Err(j) => return Err(j),
                },
                205 => match ExtendedTraceInformation::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::ExtendedTraceInformation(i));
                    }
                    Err(j) => return Err(j),
                },
                206 => match MonitoringEventExtensionInfo::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::MonitoringEventExtensionInfo(i));
                    }
                    Err(j) => return Err(j),
                },
                254 => match SpecialIEWithTypeExt::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::SpecialIEWithTypeExt(i));
                    }
                    Err(j) => return Err(j),
                },
                255 => match PrivateExtension::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::PrivateExtension(i));
                    }
                    Err(j) => return Err(j),
                },
                _ => match Unknown::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::Unknown(i));
                    }
                    Err(j) => return Err(j),
                },
            }
        }
        Ok(ies)
    }
}
