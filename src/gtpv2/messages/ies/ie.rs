/*

List of GTPv2-C IEs according to 3GPP TS 29.274 V17.10.0 (2023-12)

IE Type         Name                                                            Status
0               Reserved                                                        NA
1               International Mobile Subscriber Identity (IMSI)                 Implemented
2               Cause                                                           Implemented
3               Recovery (Restart Counter)                                      Implemented
4 to 34         Reserved for S101 interface                                     NA
35 to 50        Reserved for S121 interface                                     NA
51              STN-SR                                                          Implemented
56              SRVCC Cause                                                     Implemented
52 to 70        Reserved for Sv interface                                       NA
71              Access Point Name (APN)                                         Implemented
72              Aggregate Maximum Bit Rate (AMBR)                               Implemented
73              EPS Bearer ID (EBI)                                             Implemented
74              IP Address                                                      Implemented
75              Mobile Equipment Identity (MEI)                                 Implemented
76              MSISDN                                                          Implemented
77              Indication                                                      Implemented
78              Protocol Configuration Options (PCO)                            Implemented
79              PDN Address Allocation (PAA)                                    Implemented
80              Bearer Level Quality of Service (Bearer QoS)                    Implemented
81              Flow Quality of Service (Flow QoS)                              Implemented
82              RAT Type                                                        Implemented
83              Serving Network                                                 Implemented
84              EPS Bearer Level Traffic Flow Template (Bearer TFT)             Implemented
85              Traffic Aggregation Description (TAD)                           Implemented
86              User Location Information (ULI)                                 Implemented
87              Fully Qualified Tunnel Endpoint Identifier (F-TEID)             Implemented
88              TMSI                                                            Implemented
89              Global CN-Id                                                    Implemented
90              S103 PDN Data Forwarding Info (S103PDF)                         Implemented
91              S1-U Data Forwarding Info (S1UDF)                               Implemented
92              Delay Value                                                     Implemented
93              Bearer Context                                                  To be checked
94              Charging ID                                                     Implemented
95              Charging Characteristics                                        Implemented
96              Trace Information                                               Implemented
97              Bearer Flags                                                    Implemented
98              Reserved                                                        NA
99              PDN Type                                                        Implemented
100             Procedure Transaction ID                                        Implemented
101             Reserved                                                        NA
102             Reserved                                                        NA
103             MM Context (GSM Key and Triplets)                               Implemented
104             MM Context (UMTS Key, Used Cipher and Quintuplets)              Implemented
105             MM Context (GSM Key, Used Cipher and Quintuplets)               Implemented
106             MM Context (UMTS Key and Quintuplets)                           Implemented
107             MM Context (EPS Security Context, Quadruplets and Quintuplets)  Implemented
108             MM Context (UMTS Key, Quadruplets and Quintuplets)              Implemented
109             PDN Connection                                                  To be checked
110             PDU Numbers                                                     Implemented
111             P-TMSI                                                          Implemented
112             P-TMSI Signature                                                Implemented
113             Hop Counter                                                     Implemented
114             UE Time Zone                                                    Implemented
115             Trace Reference                                                 Implemented
116             Complete Request Message                                        Implemented
117             GUTI                                                            Implemented
118             F-Container                                                     Implemented
119             F-Cause                                                         Implemented
120             PLMN ID                                                         Implemented
121             Target Identification                                           Implemented
122             Reserved                                                        NA
123             Packet Flow ID                                                  Implemented
124             RAB Context                                                     Implemented
125             Source RNC PDCP Context Info                                    Implemented
126             Port Number                                                     Implemented
127             APN Restriction                                                 Implemented
128             Selection Mode                                                  Implemented
129             Source Identification                                           Implemented
130             Reserved                                                        NA
131             Change Reporting Action                                         Implemented
132             Fully Qualified PDN Connection Set Identifier (FQ-CSID)         Implemented
133             Channel needed                                                  Implemented
134             eMLPP Priority                                                  Implemented
135             Node Type                                                       Implemented
136             Fully Qualified Domain Name (FQDN)                              Implemented
137             Transaction Identifier (TI)                                     Implemented
138             MBMS Session Duration                                           Implemented
139             MBMS Service Area                                               Implemented
140             MBMS Session Identifier                                         Implemented
141             MBMS Flow Identifier                                            Implemented
142             MBMS IP Multicast Distribution                                  Implemented
143             MBMS Distribution Acknowledge                                   Implemented
144             RFSP Index                                                      Implemented
145             User CSG Information (UCI)                                      Implemented
146             CSG Information Reporting Action                                Implemented
147             CSG ID                                                          Implemented
148             CSG Membership Indication (CMI)                                 Implemented
149             Service indicator                                               Implemented
150             Detach Type                                                     Implemented
151             Local Distiguished Name (LDN)                                   Implemented
152             Node Features                                                   Implemented
153             MBMS Time to Data Transfer                                      Implemented
154             Throttling                                                      Implemented
155             Allocation/Retention Priority (ARP)                             Implemented
156             EPC Timer                                                       Implemented
157             Signalling Priority Indication                                  Implemented
158             Temporary Mobile Group Identity (TMGI)                          Implemented
159             Additional MM context for SRVCC                                 Implemented
160             Additional flags for SRVCC                                      Implemented
161             Reserved                                                        NA
162             MDT Configuration                                               Implemented
163             Additional Protocol Configuration Options (APCO)                Implemented
164             Absolute Time of MBMS Data Transfer                             Implemented
165             H(e)NB Information Reporting                                    Implemented
166             IPv4 Configuration Parameters (IP4CP)                           Implemented
167             Change to Report Flags                                          Implemented
168             Action Indication                                               Implemented
169             TWAN Identifier                                                 Implemented
170             ULI Timestamp                                                   Implemented
171             MBMS Flags                                                      Implemented
172             RAN/NAS Cause                                                   Implemented
173             CN Operator Selection Entity                                    Implemented
174             Trusted WLAN Mode Indication                                    Implemented
175             Node Number                                                     Implemented
176             Node Identifier                                                 Implemented
177             Presence Reporting Area Action                                  Implemented
178             Presence Reporting Area Information                             Implemented
179             TWAN Identifier Timestamp                                       Implemented
180             Overload Control Information                                    To be checked
181             Load Control Information                                        To be checked
182             Metric                                                          Implemented
183             Sequence Number                                                 Implemented
184             APN and Relative Capacity                                       Implemented
185             WLAN Offloadability Indication                                  Implemented
186             Paging and Service Information                                  Implemented
187             Integer Number                                                  Implemented
188             Millisecond Time Stamp                                          Implemented
189             Monitoring Event Information                                    Implemented
190             ECGI List                                                       Implemented
191             Remote UE Context                                               To be checked
192             Remote User ID                                                  Implemented
193             Remote UE IP information                                        Implemented
194             CIoT Optimizations Support Indication                           Implemented
195             SCEF PDN Connection                                             To be checked
196             Header Compression Configuration                                Implemented
197             Extended Protocol Configuration Options (ePCO)                  Implemented
198             Serving PLMN Rate Control                                       Implemented
199             Counter                                                         Implemented
200             Mapped UE Usage Type                                            Implemented
201             Secondary RAT Usage Data Report                                 Implemented
202             UP Function Selection Indication Flags                          Implemented
203             Maximum Packet Loss Rate                                        Implemented
204             APN Rate Control Status                                         Implemented
205             Extended Trace Information                                      Implemented
206             Monitoring Event Extension Information                          Implemented
207             Additional RRM Policy Index                                     Implemented
208             V2X Context                                                     Implemented
209             PC5 QoS Parameters                                              Implemented
210             Services Authorized                                             Implemented
211             Bit Rate                                                        Implemented
212             PC5 QoS Flow                                                    Implemented
213             SGi PtP Tunnel Address                                          Implemented
214             PGW Change Info                                                 Implemented
215             PGW FQDN                                                        Implemented
216             Group Id                                                        Implemented
217             PSCell ID                                                       Implemented
218             UP Security Policy                                              Implemented
219             Alternative IMSI                                                Implemented
220 to 253      Spare. For future use.                                          NA
254             Special IE type for IE Type Extension                           Implemented
255             Private Extension                                               Implemented
256 to 65535    Spare. For future use.                                          NA



*/

use crate::gtpv2::{errors::GTPV2Error, messages::ies::*};

#[derive(Debug, Clone, PartialEq)]
pub enum InformationElement {
    Imsi(Imsi),
    Cause(Cause),
    Recovery(Recovery),
    StnSr(StnSr),
    SrvccCause(SrvccCause),
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
    MmContext(MmContext),
    PdnConnections(Box<PdnConnections>),
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
    MbmsSa(MbmsServiceArea),
    MbmsSd(MbmsSessionDuration),
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
    AdditionalRrmPolicyIndex(AdditionalRrmPolicyIndex),
    V2xInformation(V2xInformation),
    PC5QosParameters(PC5QosParameters),
    ServicesAuthorized(ServicesAuthorized),
    BitRate(BitRate),
    PC5QosFlow(PC5QosFlow),
    SgiPtpTunnelAddress(SgiPtpTunnelAddress),
    PgwChangeInfo(PgwChangeInfo),
    PgwFqdn(PgwFqdn),
    GroupId(GroupId),
    PSCellId(PSCellId),
    UpSecurityPolicy(UpSecurityPolicy),
    AlternativeImsi(AlternativeImsi),
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
            InformationElement::SrvccCause(i) => i.marshal(buffer),
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
            InformationElement::MmContext(i) => i.marshal(buffer),
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
            InformationElement::AdditionalRrmPolicyIndex(i) => i.marshal(buffer),
            InformationElement::V2xInformation(i) => i.marshal(buffer),
            InformationElement::PC5QosParameters(i) => i.marshal(buffer),
            InformationElement::ServicesAuthorized(i) => i.marshal(buffer),
            InformationElement::BitRate(i) => i.marshal(buffer),
            InformationElement::PC5QosFlow(i) => i.marshal(buffer),
            InformationElement::SgiPtpTunnelAddress(i) => i.marshal(buffer),
            InformationElement::PgwChangeInfo(i) => i.marshal(buffer),
            InformationElement::PgwFqdn(i) => i.marshal(buffer),
            InformationElement::GroupId(i) => i.marshal(buffer),
            InformationElement::PSCellId(i) => i.marshal(buffer),
            InformationElement::UpSecurityPolicy(i) => i.marshal(buffer),
            InformationElement::AlternativeImsi(i) => i.marshal(buffer),
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
                56 => match SrvccCause::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::SrvccCause(i));
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
                103..=108 => match MmContext::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::MmContext(i));
                    }
                    Err(j) => return Err(j),
                },
                109 => match PdnConnections::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::PdnConnections(Box::new(i)));
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
                138 => match MbmsSessionDuration::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::MbmsSd(i));
                    }
                    Err(j) => return Err(j),
                },
                139 => match MbmsServiceArea::unmarshal(&buffer[cursor..]) {
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
                207 => match AdditionalRrmPolicyIndex::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::AdditionalRrmPolicyIndex(i));
                    }
                    Err(j) => return Err(j),
                },
                208 => match V2xInformation::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::V2xInformation(i));
                    }
                    Err(j) => return Err(j),
                },
                209 => match PC5QosParameters::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::PC5QosParameters(i));
                    }
                    Err(j) => return Err(j),
                },
                210 => match ServicesAuthorized::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::ServicesAuthorized(i));
                    }
                    Err(j) => return Err(j),
                },
                211 => match BitRate::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::BitRate(i));
                    }
                    Err(j) => return Err(j),
                },
                212 => match PC5QosFlow::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::PC5QosFlow(i));
                    }
                    Err(j) => return Err(j),
                },
                213 => match SgiPtpTunnelAddress::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::SgiPtpTunnelAddress(i));
                    }
                    Err(j) => return Err(j),
                },
                214 => match PgwChangeInfo::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::PgwChangeInfo(i));
                    }
                    Err(j) => return Err(j),
                },
                215 => match PgwFqdn::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::PgwFqdn(i));
                    }
                    Err(j) => return Err(j),
                },
                216 => match GroupId::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::GroupId(i));
                    }
                    Err(j) => return Err(j),
                },
                217 => match PSCellId::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::PSCellId(i));
                    }
                    Err(j) => return Err(j),
                },
                218 => match UpSecurityPolicy::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::UpSecurityPolicy(i));
                    }
                    Err(j) => return Err(j),
                },
                219 => match AlternativeImsi::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        ies.push(InformationElement::AlternativeImsi(i));
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
