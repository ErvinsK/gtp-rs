/*
Message Type value (Decimal)                                Message                                             Status

0                                                           Reserved
1                                                           Echo Request                                        Implemented
2                                                           Echo Response                                       Implemented
3                                                           Version Not Supported Indication                    Implemented
4 to 16                                                     Reserved for S101 interface
17 to 24                                                    Reserved for S121 interface
25 to 31                                                    Reserved for Sv interface

-------------SGSN/MME/ TWAN/ePDG to PGW (S4/S11, S5/S8, S2a, S2b)----------------------------

32                                                          Create Session Request                              Implemented      
33                                                          Create Session Response                             Implemented
36                                                          Delete Session Request                              Implemented
37                                                          Delete Session Response                             Implemented

-------------SGSN/MME/ePDG to PGW (S4/S11, S5/S8, S2b)---------------------------------------

34                                                          Modify Bearer Request                               Implemented
35                                                          Modify Bearer Response                              Implemented

-------------MME to PGW (S11, S5/S8)---------------------------------------------------------

40                                                          Remote UE Report Notification                       Implemented
41                                                          Remote UE Report Acknowledge                        Implemented

-------------SGSN/MME to PGW (S4/S11, S5/S8)-------------------------------------------------

38                                                          Change Notification Request                         Implemented
39                                                          Change Notification Response                        Implemented
42 to 63                                                    For future use
164                                                         Resume Notification                                 Implemented
165                                                         Resume Acknowledge                                  Implemented

-------------Messages without explicit response----------------------------------------------

64                                                          Modify Bearer Command                               Implemented
65                                                          Modify Bearer Failure Indication                    Implemented
66                                                          Delete Bearer Command                               Implemented
67                                                          Delete Bearer Failure Indication                    Implemented
68                                                          Bearer Resource Command                             Implemented
69                                                          Bearer Resource Failure Indication                  Implemented
70                                                          Downlink Data Notification Failure Indication       Implemented
71                                                          Trace Session Activation                            Implemented
72                                                          Trace Session Deactivation                          Implemented
73                                                          Stop Paging Indication                              Implemented
74 to 94                                                    For future use

-------------PGW to SGSN/MME/ TWAN/ePDG (S5/S8, S4/S11, S2a, S2b)-----------------------------

95                                                          Create Bearer Request                               Implemented
96                                                          Create Bearer Response                              Implemented
97                                                          Update Bearer Request                               Implemented
98                                                          Update Bearer Response                              Implemented
99                                                          Delete Bearer Request                               Implemented
100                                                         Delete Bearer Response                              Implemented

-------------PGW to MME, MME to PGW, SGW to PGW, SGW to MME, PGW to TWAN/ePDG, TWAN/ePDG to PGW (S5/S8, S11, S2a, S2b)

101                                                         Delete PDN Connection Set Request                   Implemented
102                                                         Delete PDN Connection Set Response                  Implemented

-------------PGW to SGSN/MME (S5, S4/S11)------------------------------------------------------

103                                                         PGW Downlink Triggering Notification                Not implemented
104                                                         PGW Downlink Triggering Acknowledge                 Not implemented
105 to 127                                                  For future use

-------------MME to MME, SGSN to MME, MME to SGSN, SGSN to SGSN, MME to AMF, AMF to MME (S3/S10/S16/N26)

128                                                         Identification Request                              Not implemented
129                                                         Identification Response                             Not implemented
130                                                         Context Request                                     Not implemented
131                                                         Context Response                                    Not implemented
132                                                         Context Acknowledge                                 Not implemented      
133                                                         Forward Relocation Request                          Not implemented
134                                                         Forward Relocation Response                         Not implemented
135                                                         Forward Relocation Complete Notification            Not implemented
136                                                         Forward Relocation Complete Acknowledge             Not implemented
137                                                         Forward Access Context Notification                 Not implemented
138                                                         Forward Access Context Acknowledge                  Not implemented
139                                                         Relocation Cancel Request                           Not implemented
140                                                         Relocation Cancel Response                          Not implemented
141                                                         Configuration Transfer Tunnel                       Not implemented   
142 to 148                                                  For future use
152                                                         RAN Information Relay                               Not implemented

------------SGSN to MME, MME to SGSN (S3)---------------------------------------------------------

149                                                         Detach Notification                                 Implemented
150                                                         Detach Acknowledge                                  Implemented
151                                                         CS Paging Indication                                Not implemented
153                                                         Alert MME Notification                              Implemented       
154                                                         Alert MME Acknowledge                               Implemented
155                                                         UE Activity Notification                            Implemented
156                                                         UE Activity Acknowledge                             Implemented
157                                                         ISR Status Indication                               Implemented
158                                                         UE Registration Query Request                       Implemented    
159                                                         UE Registration Query Response                      Implemented

-----------SGSN/MME to SGW, SGSN to MME (S4/S11/S3), SGSN to SGSN (S16), SGW to PGW (S5/S8)-------

162                                                         Suspend Notification                                Implemented
163                                                         Suspend Acknowledge                                 Implemented

-----------SGSN/MME to SGW (S4/S11)---------------------------------------------------------------

160                                                         Create Forwarding Tunnel Request                    Not implemented
161                                                         Create Forwarding Tunnel Response                   Not implemented
166                                                         Create Indirect Data Forwarding Tunnel Request      Implemented
167                                                         Create Indirect Data Forwarding Tunnel Response     Implemented
168                                                         Delete Indirect Data Forwarding Tunnel Request      Implemented
169                                                         Delete Indirect Data Forwarding Tunnel Response     Implemented
170                                                         Release Access Bearers Request                      Implemented
171                                                         Release Access Bearers Response                     Implemented
172 to 175                                                  For future use

----------SGW to SGSN/MME (S4/S11)----------------------------------------------------------------

176                                                         Downlink Data Notification                          Implemented
177                                                         Downlink Data Notification Acknowledge              Implemented
179                                                         PGW Restart Notification                            Implemented
180                                                         PGW Restart Notification Acknowledge                Implemented

----------SGW to SGSN (S4)------------------------------------------------------------------------

178                                                         Reserved. Allocated in earlier version of the specification.
181 to 199                                                  For future use

----------SGW to PGW, PGW to SGW (S5/S8)----------------------------------------------------------

200                                                         Update PDN Connection Set Request                   Implemented
201                                                         Update PDN Connection Set Response                  Implemented
202 to 210                                                  For future use

----------MME to SGW (S11)------------------------------------------------------------------------

211                                                         Modify Access Bearers Request                       Not implemented
212                                                         Modify Access Bearers Response                      Not implemented
213 to 230                                                  For future use

----------MBMS GW to MME/SGSN (Sm/Sn)-------------------------------------------------------------

231                                                         MBMS Session Start Request                          Not implemented
232                                                         MBMS Session Start Response                         Not implemented
233                                                         MBMS Session Update Request                         Not implemented
234                                                         MBMS Session Update Response                        Not implemented
235                                                         MBMS Session Stop Request                           Not implemented
236                                                         MBMS Session Stop Response                          Not implemented   
237 to 239                                                  For future use

----------Other-----------------------------------------------------------------------------------

240 to 247                                                  Reserved for Sv interface (see also types 25 to 31)
248 to 255                                                  For future use
*/



pub use {
    alertmmeacknowledge::*, alertmmenotification::*, bearerresourcecommand::*,
    bearerresourcefailureind::*, changenotificationreq::*, changenotificationresp::*, commons::*,
    createbearerreq::*, createbearerresp::*, createsessionreq::*, createsessionresp::*,
    deletebearercommand::*, deletebearerfailureind::*, deletebearerreq::*, deletebearerresp::*,
    deletesessionreq::*, deletesessionresp::*, echoreq::*, echoresp::*, ies::*,
    modifybearercommand::*, modifybearerfailureind::*, modifybearerreq::*, modifybearerresp::*,
    resumeacknowledge::*, resumenotification::*, suspendacknowledge::*, suspendnotification::*,
    ueactivityacknowledge::*, ueactivitynotification::*, updatebearerreq::*, updatebearerresp::*,
    versionnotsupported::*, createindirectdatafwtunnelreq::*, createindirectdatafwtunnelresp::*,
    deleteindirectdatafwtunnelreq::*, deleteindirectdatafwtunnelresp::*, releaseaccessbearersreq::*,
    releaseaccessbearersresp::*, stoppagingindication::*, remoteuereportnotification::*, remoteuereportacknowledge::*,
    downlinkdatanotificationfailureindication::*, detachacknowledge::*, detachnotification::*, tracesessionactivation::*,
    tracesessiondeactivation::*, deletepdnconnectionsetreq::*, deletepdnconnectionsetresp::*, updatepdnconnectionsetreq::*,
    updatepdnconnectionsetresp::*, downlinkdatanotification::*, downlinkdatanotificationacknowledge::*,
    pgwrestartnotificationacknowledge::*, pgwrestartnotification::*, isrstatusindication::*, ueregistrationqueryreq::*,
    ueregistrationqueryresp::*,
};

mod alertmmeacknowledge;
mod alertmmenotification;
mod bearerresourcecommand;
mod bearerresourcefailureind;
mod changenotificationreq;
mod changenotificationresp;
mod commons;
mod createbearerreq;
mod createbearerresp;
mod createsessionreq;
mod createsessionresp;
mod deletebearercommand;
mod deletebearerfailureind;
mod deletebearerreq;
mod deletebearerresp;
mod deletesessionreq;
mod deletesessionresp;
mod echoreq;
mod echoresp;
mod ies;
mod modifybearercommand;
mod modifybearerfailureind;
mod modifybearerreq;
mod modifybearerresp;
mod resumeacknowledge;
mod resumenotification;
mod suspendacknowledge;
mod suspendnotification;
mod ueactivityacknowledge;
mod ueactivitynotification;
mod updatebearerreq;
mod updatebearerresp;
mod versionnotsupported;
mod createindirectdatafwtunnelreq;
mod createindirectdatafwtunnelresp;
mod deleteindirectdatafwtunnelreq;
mod deleteindirectdatafwtunnelresp;
mod releaseaccessbearersreq;
mod releaseaccessbearersresp;
mod stoppagingindication;
mod remoteuereportnotification;
mod remoteuereportacknowledge;
mod downlinkdatanotificationfailureindication;
mod detachacknowledge;
mod detachnotification;
mod tracesessionactivation;
mod tracesessiondeactivation;
mod deletepdnconnectionsetreq;
mod deletepdnconnectionsetresp;
mod updatepdnconnectionsetreq;
mod updatepdnconnectionsetresp;
mod downlinkdatanotification;
mod downlinkdatanotificationacknowledge;
mod pgwrestartnotificationacknowledge;
mod pgwrestartnotification;
mod isrstatusindication;
mod ueregistrationqueryreq;
mod ueregistrationqueryresp;