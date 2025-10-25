/*
Message Type value (Decimal)                                Message                                             Status

0                                                           Reserved
1                                                           Echo Request                                        Implemented*
2                                                           Echo Response                                       Implemented*
3                                                           Version Not Supported Indication                    Implemented*
4 to 16                                                     Reserved for S101 interface
17 to 24                                                    Reserved for S121 interface
25 to 31                                                    Reserved for Sv interface

-------------SGSN/MME/ TWAN/ePDG to PGW (S4/S11, S5/S8, S2a, S2b)----------------------------

32                                                          Create Session Request                              Implemented*
33                                                          Create Session Response                             Implemented*
36                                                          Delete Session Request                              Implemented*
37                                                          Delete Session Response                             Implemented*

-------------SGSN/MME/ePDG to PGW (S4/S11, S5/S8, S2b)---------------------------------------

34                                                          Modify Bearer Request                               Implemented*
35                                                          Modify Bearer Response                              Implemented*

-------------MME to PGW (S11, S5/S8)---------------------------------------------------------

40                                                          Remote UE Report Notification                       Implemented*
41                                                          Remote UE Report Acknowledge                        Implemented*

-------------SGSN/MME to PGW (S4/S11, S5/S8)-------------------------------------------------

38                                                          Change Notification Request                         Implemented*
39                                                          Change Notification Response                        Implemented*
42 to 63                                                    For future use
164                                                         Resume Notification                                 Implemented*
165                                                         Resume Acknowledge                                  Implemented*

-------------Messages without explicit response----------------------------------------------

64                                                          Modify Bearer Command                               Implemented*
65                                                          Modify Bearer Failure Indication                    Implemented*
66                                                          Delete Bearer Command                               Implemented*
67                                                          Delete Bearer Failure Indication                    Implemented*
68                                                          Bearer Resource Command                             Implemented*
69                                                          Bearer Resource Failure Indication                  Implemented*
70                                                          Downlink Data Notification Failure Indication       Implemented*
71                                                          Trace Session Activation                            Implemented*
72                                                          Trace Session Deactivation                          Implemented*
73                                                          Stop Paging Indication                              Implemented*
74 to 94                                                    For future use

-------------PGW to SGSN/MME/ TWAN/ePDG (S5/S8, S4/S11, S2a, S2b)-----------------------------

95                                                          Create Bearer Request                               Implemented*
96                                                          Create Bearer Response                              Implemented*
97                                                          Update Bearer Request                               Implemented*
98                                                          Update Bearer Response                              Implemented*
99                                                          Delete Bearer Request                               Implemented*
100                                                         Delete Bearer Response                              Implemented*

-------------PGW to MME, MME to PGW, SGW to PGW, SGW to MME, PGW to TWAN/ePDG, TWAN/ePDG to PGW (S5/S8, S11, S2a, S2b)

101                                                         Delete PDN Connection Set Request                   Implemented*
102                                                         Delete PDN Connection Set Response                  Implemented*

-------------PGW to SGSN/MME (S5, S4/S11)------------------------------------------------------

103                                                         PGW Downlink Triggering Notification                Implemented*
104                                                         PGW Downlink Triggering Acknowledge                 Implemented*
105 to 127                                                  For future use

-------------MME to MME, SGSN to MME, MME to SGSN, SGSN to SGSN, MME to AMF, AMF to MME (S3/S10/S16/N26)

128                                                         Identification Request                              Implemented*
129                                                         Identification Response                             Implemented*
130                                                         Context Request                                     Implemented*
131                                                         Context Response                                    Implemented*
132                                                         Context Acknowledge                                 Implemented*
133                                                         Forward Relocation Request                          Implemented*
134                                                         Forward Relocation Response                         Implemented*
135                                                         Forward Relocation Complete Notification            Implemented*
136                                                         Forward Relocation Complete Acknowledge             Implemented*
137                                                         Forward Access Context Notification                 Implemented*
138                                                         Forward Access Context Acknowledge                  Implemented*
139                                                         Relocation Cancel Request                           Implemented*
140                                                         Relocation Cancel Response                          Implemented*
141                                                         Configuration Transfer Tunnel                       Implemented*
142 to 148                                                  For future use
152                                                         RAN Information Relay                               Implemented*

------------SGSN to MME, MME to SGSN (S3)---------------------------------------------------------

149                                                         Detach Notification                                 Implemented*
150                                                         Detach Acknowledge                                  Implemented*
151                                                         CS Paging Indication                                Implemented*
153                                                         Alert MME Notification                              Implemented*
154                                                         Alert MME Acknowledge                               Implemented*
155                                                         UE Activity Notification                            Implemented*
156                                                         UE Activity Acknowledge                             Implemented*
157                                                         ISR Status Indication                               Implemented*
158                                                         UE Registration Query Request                       Implemented*
159                                                         UE Registration Query Response                      Implemented*

-----------SGSN/MME to SGW, SGSN to MME (S4/S11/S3), SGSN to SGSN (S16), SGW to PGW (S5/S8)-------

162                                                         Suspend Notification                                Implemented*
163                                                         Suspend Acknowledge                                 Implemented*

-----------SGSN/MME to SGW (S4/S11)---------------------------------------------------------------

160                                                         Create Forwarding Tunnel Request                    Implemented*
161                                                         Create Forwarding Tunnel Response                   Implemented*
166                                                         Create Indirect Data Forwarding Tunnel Request      Implemented*
167                                                         Create Indirect Data Forwarding Tunnel Response     Implemented*
168                                                         Delete Indirect Data Forwarding Tunnel Request      Implemented*
169                                                         Delete Indirect Data Forwarding Tunnel Response     Implemented*
170                                                         Release Access Bearers Request                      Implemented*
171                                                         Release Access Bearers Response                     Implemented*
172 to 175                                                  For future use

----------SGW to SGSN/MME (S4/S11)----------------------------------------------------------------

176                                                         Downlink Data Notification                          Implemented*
177                                                         Downlink Data Notification Acknowledge              Implemented*
179                                                         PGW Restart Notification                            Implemented*
180                                                         PGW Restart Notification Acknowledge                Implemented*

----------SGW to SGSN (S4)------------------------------------------------------------------------

178                                                         Reserved. Allocated in earlier version of the specification.
181 to 199                                                  For future use

----------SGW to PGW, PGW to SGW (S5/S8)----------------------------------------------------------

200                                                         Update PDN Connection Set Request                   Implemented*
201                                                         Update PDN Connection Set Response                  Implemented*
202 to 210                                                  For future use

----------MME to SGW (S11)------------------------------------------------------------------------

211                                                         Modify Access Bearers Request                       Implemented*
212                                                         Modify Access Bearers Response                      Implemented*
213 to 230                                                  For future use

----------MBMS GW to MME/SGSN (Sm/Sn)-------------------------------------------------------------

231                                                         MBMS Session Start Request                          Implemented*
232                                                         MBMS Session Start Response                         Implemented*
233                                                         MBMS Session Update Request                         Implemented*
234                                                         MBMS Session Update Response                        Implemented*
235                                                         MBMS Session Stop Request                           Implemented*
236                                                         MBMS Session Stop Response                          Implemented*
237 to 239                                                  For future use

----------Other-----------------------------------------------------------------------------------

240 to 247                                                  Reserved for Sv interface (see also types 25 to 31)
248 to 255                                                  For future use
*/

pub use {
    alertmmeacknowledge::*, alertmmenotification::*, bearerresourcecommand::*,
    bearerresourcefailureind::*, changenotificationreq::*, changenotificationresp::*, commons::*,
    contextacknowledge::*, contextreq::*, contextresp::*, createbearerreq::*, createbearerresp::*,
    createforwardingtunnelreq::*, createforwardingtunnelresp::*, createindirectdatafwtunnelreq::*,
    createindirectdatafwtunnelresp::*, createsessionreq::*, createsessionresp::*,
    cspagingindication::*, deletebearercommand::*, deletebearerfailureind::*, deletebearerreq::*,
    deletebearerresp::*, deleteindirectdatafwtunnelreq::*, deleteindirectdatafwtunnelresp::*,
    deletepdnconnectionsetreq::*, deletepdnconnectionsetresp::*, deletesessionreq::*,
    deletesessionresp::*, detachacknowledge::*, detachnotification::*, downlinkdatanotification::*,
    downlinkdatanotificationacknowledge::*, downlinkdatanotificationfailureindication::*,
    echoreq::*, echoresp::*, forwardaccesscontextacknowledge::*,
    forwardaccesscontextnotification::*, forwardrelocationcompleteacknowledge::*,
    forwardrelocationcompletenotification::*, forwardrelocationreq::*, forwardrelocationresp::*,
    identificationreq::*, identificationresp::*, ies::*, isrstatusindication::*,
    mbmssessionstartreq::*, mbmssessionstartresp::*, mbmssessionstopreq::*, mbmssessionstopresp::*,
    mbmssessionupdatereq::*, mbmssessionupdateresp::*, modifyaccessbearersreq::*,
    modifyaccessbearersresp::*, modifybearercommand::*, modifybearerfailureind::*,
    modifybearerreq::*, modifybearerresp::*, pgwdownlinktriggeringacknowledge::*,
    pgwdownlinktriggeringnotification::*, pgwrestartnotification::*,
    pgwrestartnotificationacknowledge::*, raninformationrelay::*, releaseaccessbearersreq::*,
    releaseaccessbearersresp::*, relocationcancelreq::*, relocationcancelresp::*,
    remoteuereportacknowledge::*, remoteuereportnotification::*, resumeacknowledge::*,
    resumenotification::*, stoppagingindication::*, suspendacknowledge::*, suspendnotification::*,
    tracesessionactivation::*, tracesessiondeactivation::*, ueactivityacknowledge::*,
    ueactivitynotification::*, ueregistrationqueryreq::*, ueregistrationqueryresp::*,
    updatebearerreq::*, updatebearerresp::*, updatepdnconnectionsetreq::*,
    updatepdnconnectionsetresp::*, versionnotsupported::*,
};

mod alertmmeacknowledge;
mod alertmmenotification;
mod bearerresourcecommand;
mod bearerresourcefailureind;
mod changenotificationreq;
mod changenotificationresp;
mod commons;
mod contextacknowledge;
mod contextreq;
mod contextresp;
mod createbearerreq;
mod createbearerresp;
mod createforwardingtunnelreq;
mod createforwardingtunnelresp;
mod createindirectdatafwtunnelreq;
mod createindirectdatafwtunnelresp;
mod createsessionreq;
mod createsessionresp;
mod cspagingindication;
mod deletebearercommand;
mod deletebearerfailureind;
mod deletebearerreq;
mod deletebearerresp;
mod deleteindirectdatafwtunnelreq;
mod deleteindirectdatafwtunnelresp;
mod deletepdnconnectionsetreq;
mod deletepdnconnectionsetresp;
mod deletesessionreq;
mod deletesessionresp;
mod detachacknowledge;
mod detachnotification;
mod downlinkdatanotification;
mod downlinkdatanotificationacknowledge;
mod downlinkdatanotificationfailureindication;
mod echoreq;
mod echoresp;
mod forwardaccesscontextacknowledge;
mod forwardaccesscontextnotification;
mod forwardrelocationcompleteacknowledge;
mod forwardrelocationcompletenotification;
mod forwardrelocationreq;
mod forwardrelocationresp;
mod identificationreq;
mod identificationresp;
mod ies;
mod isrstatusindication;
mod mbmssessionstartreq;
mod mbmssessionstartresp;
mod mbmssessionstopreq;
mod mbmssessionstopresp;
mod mbmssessionupdatereq;
mod mbmssessionupdateresp;
mod modifyaccessbearersreq;
mod modifyaccessbearersresp;
mod modifybearercommand;
mod modifybearerfailureind;
mod modifybearerreq;
mod modifybearerresp;
mod pgwdownlinktriggeringacknowledge;
mod pgwdownlinktriggeringnotification;
mod pgwrestartnotification;
mod pgwrestartnotificationacknowledge;
mod raninformationrelay;
mod releaseaccessbearersreq;
mod releaseaccessbearersresp;
mod relocationcancelreq;
mod relocationcancelresp;
mod remoteuereportacknowledge;
mod remoteuereportnotification;
mod resumeacknowledge;
mod resumenotification;
mod stoppagingindication;
mod suspendacknowledge;
mod suspendnotification;
mod tracesessionactivation;
mod tracesessiondeactivation;
mod ueactivityacknowledge;
mod ueactivitynotification;
mod ueregistrationqueryreq;
mod ueregistrationqueryresp;
mod updatebearerreq;
mod updatebearerresp;
mod updatepdnconnectionsetreq;
mod updatepdnconnectionsetresp;
mod versionnotsupported;
