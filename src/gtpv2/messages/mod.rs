/*
Message Type value (Decimal)                                Message

0                                                           Reserved
1                                                           Echo Request
2                                                           Echo Response
3                                                           Version Not Supported Indication
4 to 16                                                     Reserved for S101 interface
17 to 24                                                    Reserved for S121 interface
25 to 31                                                    Reserved for Sv interface

-------------SGSN/MME/ TWAN/ePDG to PGW (S4/S11, S5/S8, S2a, S2b)----------------------------

32                                                          Create Session Request
33                                                          Create Session Response
36                                                          Delete Session Request
37                                                          Delete Session Response

-------------SGSN/MME/ePDG to PGW (S4/S11, S5/S8, S2b)---------------------------------------

34                                                          Modify Bearer Request
35                                                          Modify Bearer Response

-------------MME to PGW (S11, S5/S8)---------------------------------------------------------

40                                                          Remote UE Report Notification
41                                                          Remote UE Report Acknowledge

-------------SGSN/MME to PGW (S4/S11, S5/S8)-------------------------------------------------

38                                                          Change Notification Request
39                                                          Change Notification Response
42 to 63                                                    For future use
164                                                         Resume Notification
165                                                         Resume Acknowledge

-------------Messages without explicit response----------------------------------------------

64                                                          Modify Bearer Command
65                                                          Modify Bearer Failure Indication
66                                                          Delete Bearer Command
67                                                          Delete Bearer Failure Indication
68                                                          Bearer Resource Command
69                                                          Bearer Resource Failure Indication
70                                                          Downlink Data Notification Failure Indication
71                                                          Trace Session Activation
72                                                          Trace Session Deactivation
73                                                          Stop Paging Indication
74 to 94                                                    For future use

-------------PGW to SGSN/MME/ TWAN/ePDG (S5/S8, S4/S11, S2a, S2b)-----------------------------

95                                                          Create Bearer Request
96                                                          Create Bearer Response
97                                                          Update Bearer Request
98                                                          Update Bearer Response
99                                                          Delete Bearer Request
100                                                         Delete Bearer Response

-------------PGW to MME, MME to PGW, SGW to PGW, SGW to MME, PGW to TWAN/ePDG, TWAN/ePDG to PGW (S5/S8, S11, S2a, S2b)

101                                                         Delete PDN Connection Set Request
102                                                         Delete PDN Connection Set Response

-------------PGW to SGSN/MME (S5, S4/S11)------------------------------------------------------

103                                                         PGW Downlink Triggering Notification
104                                                         PGW Downlink Triggering Acknowledge
105 to 127                                                  For future use

-------------MME to MME, SGSN to MME, MME to SGSN, SGSN to SGSN, MME to AMF, AMF to MME (S3/S10/S16/N26)

128                                                         Identification Request
129                                                         Identification Response
130                                                         Context Request
131                                                         Context Response
132                                                         Context Acknowledge
133                                                         Forward Relocation Request
134                                                         Forward Relocation Response
135                                                         Forward Relocation Complete Notification
136                                                         Forward Relocation Complete Acknowledge
137                                                         Forward Access Context Notification
138                                                         Forward Access Context Acknowledge
139                                                         Relocation Cancel Request
140                                                         Relocation Cancel Response
141                                                         Configuration Transfer Tunnel
142 to 148                                                  For future use
152                                                         RAN Information Relay

------------SGSN to MME, MME to SGSN (S3)---------------------------------------------------------

149                                                         Detach Notification
150                                                         Detach Acknowledge
151                                                         CS Paging Indication
153                                                         Alert MME Notification
154                                                         Alert MME Acknowledge
155                                                         UE Activity Notification
156                                                         UE Activity Acknowledge
157                                                         ISR Status Indication
158                                                         UE Registration Query Request
159                                                         UE Registration Query Response

-----------SGSN/MME to SGW, SGSN to MME (S4/S11/S3), SGSN to SGSN (S16), SGW to PGW (S5/S8)-------

162                                                         Suspend Notification
163                                                         Suspend Acknowledge

-----------SGSN/MME to SGW (S4/S11)---------------------------------------------------------------

160                                                         Create Forwarding Tunnel Request
161                                                         Create Forwarding Tunnel Response
166                                                         Create Indirect Data Forwarding Tunnel Request
167                                                         Create Indirect Data Forwarding Tunnel Response
168                                                         Delete Indirect Data Forwarding Tunnel Request
169                                                         Delete Indirect Data Forwarding Tunnel Response
170                                                         Release Access Bearers Request
171                                                         Release Access Bearers Response
172 to 175                                                  For future use

----------SGW to SGSN/MME (S4/S11)----------------------------------------------------------------

176                                                         Downlink Data Notification
177                                                         Downlink Data Notification Acknowledge
179                                                         PGW Restart Notification
180                                                         PGW Restart Notification Acknowledge

----------SGW to SGSN (S4)------------------------------------------------------------------------

178                                                         Reserved. Allocated in earlier version of the specification.
181 to 199                                                  For future use

----------SGW to PGW, PGW to SGW (S5/S8)----------------------------------------------------------

200                                                         Update PDN Connection Set Request
201                                                         Update PDN Connection Set Response
202 to 210                                                  For future use

----------MME to SGW (S11)------------------------------------------------------------------------

211                                                         Modify Access Bearers Request
212                                                         Modify Access Bearers Response
213 to 230                                                  For future use

----------MBMS GW to MME/SGSN (Sm/Sn)-------------------------------------------------------------

231                                                         MBMS Session Start Request
232                                                         MBMS Session Start Response
233                                                         MBMS Session Update Request
234                                                         MBMS Session Update Response
235                                                         MBMS Session Stop Request
236                                                         MBMS Session Stop Response
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
    versionnotsupported::*, createindirectdatafwtunnelreq::*,
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
