use crate::gtpv1::errors::*;
use crate::gtpv1::gtpc::header::*;
use crate::gtpv1::gtpc::messages::{commons::*, *};
use crate::gtpv1::utils::*;
use std::collections::HashMap;

// According to 3GPP TS 29.060 V15.5.0 (2019-06)
pub const MS_INFO_CHANGE_NOTIFICATION_RESPONSE: u8 = 129;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MSInfoChangeNotificationResponse {
    pub header: Gtpv1Header,
    pub cause: Cause,
    pub imsi: Option<Imsi>,
    pub linked_nsapi: Option<Nsapi>,
    pub imei: Option<Imei>,
    pub ms_info_change: Option<MSInfoChangeReportingAction>,
    pub csg_info_report: Option<CSGInformationReportingAction>,
    pub private_extension: Option<PrivateExtension>,
}

impl Default for MSInfoChangeNotificationResponse {
    fn default() -> Self {
        let header = Gtpv1Header {
            msgtype: MS_INFO_CHANGE_NOTIFICATION_RESPONSE,
            ..Default::default()
        };
        MSInfoChangeNotificationResponse {
            header,
            cause: Cause::default(),
            imsi: None,
            linked_nsapi: None,
            imei: None,
            ms_info_change: None,
            csg_info_report: None,
            private_extension: None,
        }
    }
}

impl Messages for MSInfoChangeNotificationResponse {
    fn marshal(self, buffer: &mut Vec<u8>) {
        // Marshal header
        self.header.marshal(buffer);

        // Marshal Cause IE
        self.cause.marshal(buffer);

        // Marshal IMSI IE
        if let Some(i) = self.imsi {
            i.marshal(buffer);
        }

        // Marshal Linked NSAPI IE
        if let Some(i) = self.linked_nsapi {
            i.marshal(buffer);
        }

        // Marshal IMEI IE
        if let Some(i) = self.imei {
            i.marshal(buffer);
        }

        // Marshal MS Info Change Reporting Action IE
        if let Some(i) = self.ms_info_change {
            i.marshal(buffer);
        }

        // Marshal CSG Information Reporting Action IE
        if let Some(i) = self.csg_info_report {
            i.marshal(buffer);
        }

        // Marshal Private Extension IE
        if let Some(i) = self.private_extension {
            i.marshal(buffer);
        }

        set_length(buffer);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV1Error> {
        let mut msg_hash: HashMap<u8, u8> = HashMap::new();

        let mut message = MSInfoChangeNotificationResponse::default();

        match Gtpv1Header::unmarshal(buffer) {
            Ok(h) => message.header = h,
            Err(e) => return Err(e),
        }

        if message.header.msgtype != MS_INFO_CHANGE_NOTIFICATION_RESPONSE {
            return Err(GTPV1Error::MessageIncorrectMessageType);
        }

        if (message.header.length + 8) as usize > buffer.len() {
            return Err(GTPV1Error::MessageLengthError);
        }

        let mut cursor = message.header.len();
        let mut prev_ie: u8 = 0;
        loop {
            if cursor >= buffer.len() {
                break;
            }
            if buffer[cursor] < prev_ie {
                return Err(GTPV1Error::MessageInvalidMessageFormat);
            }
            prev_ie = buffer[cursor];
            let current_byte = buffer[cursor];
            match current_byte {
                CAUSE => match Cause::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        msg_hash
                            .entry(current_byte)
                            .and_modify(|e| *e += 1)
                            .or_insert_with(|| {
                                message.cause = i;
                                1
                            });
                    }
                    Err(_) => return Err(GTPV1Error::MessageMandatoryIEMissing),
                },
                IMSI => match Imsi::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        msg_hash
                            .entry(current_byte)
                            .and_modify(|e| *e += 1)
                            .or_insert_with(|| {
                                message.imsi = Some(i);
                                1
                            });
                    }
                    Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                },
                NSAPI => match Nsapi::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        msg_hash
                            .entry(current_byte)
                            .and_modify(|e| *e += 1)
                            .or_insert_with(|| {
                                message.linked_nsapi = Some(i);
                                1
                            });
                    }
                    Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                },
                IMEI => match Imei::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        msg_hash
                            .entry(current_byte)
                            .and_modify(|e| *e += 1)
                            .or_insert_with(|| {
                                message.imei = Some(i);
                                1
                            });
                    }
                    Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                },
                MSINFO_CHANGE => match MSInfoChangeReportingAction::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        msg_hash
                            .entry(current_byte)
                            .and_modify(|e| *e += 1)
                            .or_insert_with(|| {
                                message.ms_info_change = Some(i);
                                1
                            });
                    }
                    Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                },
                CSG_INFO_REPORT => {
                    match CSGInformationReportingAction::unmarshal(&buffer[cursor..]) {
                        Ok(i) => {
                            cursor += i.len();
                            msg_hash
                                .entry(current_byte)
                                .and_modify(|e| *e += 1)
                                .or_insert_with(|| {
                                    message.csg_info_report = Some(i);
                                    1
                                });
                        }
                        Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                    }
                }
                PRIVATE_EXTENSION => match PrivateExtension::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        msg_hash
                            .entry(current_byte)
                            .and_modify(|e| *e += 1)
                            .or_insert_with(|| {
                                message.private_extension = Some(i);
                                1
                            });
                    }
                    Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                },
                _ => return Err(GTPV1Error::MessageInvalidMessageFormat),
            }
        }
        match msg_hash.get(&CAUSE) {
            Some(_) => Ok(message),
            _ => Err(GTPV1Error::MessageMandatoryIEMissing),
        }
    }
}

#[test]
fn ms_info_change_notification_resp_unmarshal_test() {
    let encoded = &[
        0x32, 0x81, 0x0, 0x1c, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x0, 0x0, 0x1, 0x80, 0x2, 0x9,
        0x41, 0x50, 0x1, 0x71, 0x44, 0x45, 0xf6, 0x14, 0x0, 0x9a, 0x0, 0x8, 0x68, 0x99, 0x15, 0x30,
        0x91, 0x64, 0x10, 0x10,
    ];
    let decoded = MSInfoChangeNotificationResponse {
        header: Gtpv1Header {
            msgtype: MS_INFO_CHANGE_NOTIFICATION_RESPONSE,
            length: 28,
            teid: 926465914,
            sequence_number: Some(39887),
            npdu_number: None,
            extension_headers: None,
        },
        cause: Cause {
            value: 128,
            ..Default::default()
        },
        imsi: Some(Imsi {
            imsi: "901405101744546".to_string(),
            ..Default::default()
        }),
        linked_nsapi: Some(Nsapi::default()),
        imei: Some(Imei {
            imei: "8699510319460101".to_string(),
            ..Default::default()
        }),
        private_extension: None,
        ms_info_change: None,
        csg_info_report: None,
    };

    assert_eq!(
        MSInfoChangeNotificationResponse::unmarshal(encoded).unwrap(),
        decoded
    );
}

#[test]
fn ms_info_change_notification_resp_marshal_test() {
    let encoded = &[
        0x32, 0x81, 0x0, 0x1c, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x0, 0x0, 0x1, 0x80, 0x2, 0x9,
        0x41, 0x50, 0x1, 0x71, 0x44, 0x45, 0xf6, 0x14, 0x0, 0x9a, 0x0, 0x8, 0x68, 0x99, 0x15, 0x30,
        0x91, 0x64, 0x10, 0x10,
    ];
    let decoded = MSInfoChangeNotificationResponse {
        header: Gtpv1Header {
            msgtype: MS_INFO_CHANGE_NOTIFICATION_RESPONSE,
            length: 0,
            teid: 926465914,
            sequence_number: Some(39887),
            npdu_number: None,
            extension_headers: None,
        },
        cause: Cause {
            value: 128,
            ..Default::default()
        },
        imsi: Some(Imsi {
            imsi: "901405101744546".to_string(),
            ..Default::default()
        }),
        linked_nsapi: Some(Nsapi::default()),
        imei: Some(Imei {
            imei: "8699510319460101".to_string(),
            ..Default::default()
        }),
        private_extension: None,
        ms_info_change: None,
        csg_info_report: None,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn ms_info_change_notification_resp_wrong_ie_order_unmarshal_test() {
    let encoded = &[
        0x32, 0x81, 0x0, 0x1c, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x0, 0x0, 0x2, 0x9, 0x41, 0x50,
        0x1, 0x71, 0x44, 0x45, 0xf6, 0x1, 0x80, 0x14, 0x0, 0x9a, 0x0, 0x8, 0x68, 0x99, 0x15, 0x30,
        0x91, 0x64, 0x10, 0x10,
    ];
    assert_eq!(
        MSInfoChangeNotificationResponse::unmarshal(encoded),
        Err(GTPV1Error::MessageInvalidMessageFormat)
    );
}

#[test]
fn ms_info_change_notification_resp_missing_mandatory_ie_unmarshal_test() {
    let encoded = &[
        0x32, 0x81, 0x0, 0x1a, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x0, 0x0, 0x2, 0x9, 0x41, 0x50,
        0x1, 0x71, 0x44, 0x45, 0xf6, 0x14, 0x0, 0x9a, 0x0, 0x8, 0x68, 0x99, 0x15, 0x30, 0x91, 0x64,
        0x10, 0x10,
    ];
    assert_eq!(
        MSInfoChangeNotificationResponse::unmarshal(encoded),
        Err(GTPV1Error::MessageMandatoryIEMissing)
    );
}
