use crate::gtpv1::errors::*;
use crate::gtpv1::gtpc::header::*;
use crate::gtpv1::gtpc::messages::{commons::*, *};
use crate::gtpv1::utils::*;
use std::collections::HashMap;

// According to 3GPP TS 29.060 V15.5.0 (2019-06)
pub const MS_INFO_CHANGE_NOTIFICATION_REQUEST: u8 = 128;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MSInfoChangeNotificationRequest {
    pub header: Gtpv1Header,
    pub imsi: Option<Imsi>,
    pub linked_nsapi: Option<Nsapi>,
    pub rat_type: RatType,
    pub uli: Option<Uli>,
    pub imei: Option<Imei>,
    pub ext_common_flags: Option<ExtendedCommonFlags>,
    pub user_csg_info: Option<Uci>,
    pub private_extension: Option<PrivateExtension>,
}

impl Default for MSInfoChangeNotificationRequest {
    fn default() -> Self {
        let header = Gtpv1Header {
            msgtype: MS_INFO_CHANGE_NOTIFICATION_REQUEST,
            ..Default::default()
        };
        MSInfoChangeNotificationRequest {
            header,
            imsi: None,
            linked_nsapi: None,
            rat_type: RatType::default(),
            uli: None,
            imei: None,
            ext_common_flags: None,
            user_csg_info: None,
            private_extension: None,
        }
    }
}

impl Messages for MSInfoChangeNotificationRequest {
    fn marshal(self, buffer: &mut Vec<u8>) {
        // Marshal header
        self.header.marshal(buffer);

        // Marshal IMSI IE
        if let Some(i) = self.imsi {
            i.marshal(buffer);
        }

        // Marshal Linked NSAPI IE
        if let Some(i) = self.linked_nsapi {
            i.marshal(buffer);
        }

        // Marshal RAT Type IE
        self.rat_type.marshal(buffer);

        // Marshal ULI IE
        if let Some(i) = self.uli {
            i.marshal(buffer);
        }

        // Marshal IMEI IE
        if let Some(i) = self.imei {
            i.marshal(buffer);
        }

        // Marshal Extended Common Flags IE
        if let Some(i) = self.ext_common_flags {
            i.marshal(buffer);
        }

        // Marshal User CSG Information IE
        if let Some(i) = self.user_csg_info {
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

        let mut message = MSInfoChangeNotificationRequest::default();

        match Gtpv1Header::unmarshal(&buffer) {
            Ok(h) => message.header = h,
            Err(e) => return Err(e),
        }

        if message.header.msgtype != MS_INFO_CHANGE_NOTIFICATION_REQUEST {
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
            dbg!(current_byte);
            match current_byte {
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
                RATTYPE => match RatType::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        msg_hash
                            .entry(current_byte)
                            .and_modify(|e| *e += 1)
                            .or_insert_with(|| {
                                message.rat_type = i;
                                1
                            });
                    }
                    Err(_) => return Err(GTPV1Error::MessageMandatoryIEMissing),
                },
                ULI => match Uli::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        msg_hash
                            .entry(current_byte)
                            .and_modify(|e| *e += 1)
                            .or_insert_with(|| {
                                message.uli = Some(i);
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
                EXTCOMMONFLAGS => match ExtendedCommonFlags::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        msg_hash
                            .entry(current_byte)
                            .and_modify(|e| *e += 1)
                            .or_insert_with(|| {
                                message.ext_common_flags = Some(i);
                                1
                            });
                    }
                    Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                },
                UCI => match Uci::unmarshal(&buffer[cursor..]) {
                    Ok(i) => {
                        cursor += i.len();
                        msg_hash
                            .entry(current_byte)
                            .and_modify(|e| *e += 1)
                            .or_insert_with(|| {
                                message.user_csg_info = Some(i);
                                1
                            });
                    }
                    Err(_) => return Err(GTPV1Error::MessageOptionalIEIncorrect),
                },
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
        match msg_hash.get(&RATTYPE) {
            Some(_) => Ok(message),
            _ => Err(GTPV1Error::MessageMandatoryIEMissing),
        }
    }
}

#[test]
fn ms_info_change_notification_req_unmarshal_test() {
    let encoded = &[
        0x32, 0x80, 0x0, 0x29, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x0, 0x0, 0x2, 0x9, 0x41, 0x50,
        0x1, 0x71, 0x44, 0x45, 0xf6, 0x14, 0x0, 0x97, 0x0, 0x1, 0x2, 0x98, 0x0, 0x8, 0x0, 0x13,
        0x0, 0x62, 0x53, 0x17, 0x4, 0x27, 0x9a, 0x0, 0x8, 0x68, 0x99, 0x15, 0x30, 0x91, 0x64, 0x10,
        0x10,
    ];
    let decoded = MSInfoChangeNotificationRequest {
        header: Gtpv1Header {
            msgtype: MS_INFO_CHANGE_NOTIFICATION_REQUEST,
            length: 41,
            teid: 926465914,
            sequence_number: Some(39887),
            npdu_number: None,
            extension_headers: None,
        },
        imsi: Some(Imsi {
            imsi: "901405101744546".to_string(),
            ..Default::default()
        }),
        linked_nsapi: Some(Nsapi::default()),
        rat_type: RatType::default(),
        uli: Some(Uli {
            mcc: 310,
            mnc: 260,
            lac: 21271,
            loc: Location::Ci(1063),
            ..Default::default()
        }),
        imei: Some(Imei {
            imei: "8699510319460101".to_string(),
            ..Default::default()
        }),
        ext_common_flags: None,
        user_csg_info: None,
        private_extension: None,
    };

    assert_eq!(
        MSInfoChangeNotificationRequest::unmarshal(encoded).unwrap(),
        decoded
    );
}

#[test]
fn ms_info_change_notification_req_marshal_test() {
    let encoded = &[
        0x32, 0x80, 0x0, 0x29, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x0, 0x0, 0x2, 0x9, 0x41, 0x50,
        0x1, 0x71, 0x44, 0x45, 0xf6, 0x14, 0x0, 0x97, 0x0, 0x1, 0x2, 0x98, 0x0, 0x8, 0x0, 0x13,
        0x0, 0x62, 0x53, 0x17, 0x4, 0x27, 0x9a, 0x0, 0x8, 0x68, 0x99, 0x15, 0x30, 0x91, 0x64, 0x10,
        0x10,
    ];
    let decoded = MSInfoChangeNotificationRequest {
        header: Gtpv1Header {
            msgtype: MS_INFO_CHANGE_NOTIFICATION_REQUEST,
            length: 0,
            teid: 926465914,
            sequence_number: Some(39887),
            npdu_number: None,
            extension_headers: None,
        },
        imsi: Some(Imsi {
            imsi: "901405101744546".to_string(),
            ..Default::default()
        }),
        linked_nsapi: Some(Nsapi::default()),
        rat_type: RatType::default(),
        uli: Some(Uli {
            mcc: 310,
            mnc: 260,
            lac: 21271,
            loc: Location::Ci(1063),
            ..Default::default()
        }),
        imei: Some(Imei {
            imei: "8699510319460101".to_string(),
            ..Default::default()
        }),
        ext_common_flags: None,
        user_csg_info: None,
        private_extension: None,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn ms_info_change_notification_req_wrong_ie_order_unmarshal_test() {
    let encoded = &[
        0x32, 0x80, 0x0, 0x29, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x0, 0x0, 0x2, 0x9, 0x41, 0x50,
        0x1, 0x71, 0x44, 0x45, 0xf6, 0x97, 0x0, 0x1, 0x2, 0x14, 0x0, 0x98, 0x0, 0x8, 0x0, 0x13,
        0x0, 0x62, 0x53, 0x17, 0x4, 0x27, 0x9a, 0x0, 0x8, 0x68, 0x99, 0x15, 0x30, 0x91, 0x64, 0x10,
        0x10,
    ];
    assert_eq!(
        MSInfoChangeNotificationRequest::unmarshal(encoded),
        Err(GTPV1Error::MessageInvalidMessageFormat)
    );
}

#[test]
fn ms_info_change_notification_req_missing_mandatory_ie_unmarshal_test() {
    let encoded = &[
        0x32, 0x80, 0x0, 0x25, 0x37, 0x38, 0xbf, 0x7a, 0x9b, 0xcf, 0x0, 0x0, 0x2, 0x9, 0x41, 0x50,
        0x1, 0x71, 0x44, 0x45, 0xf6, 0x14, 0x0, 0x98, 0x0, 0x8, 0x0, 0x13, 0x0, 0x62, 0x53, 0x17,
        0x4, 0x27, 0x9a, 0x0, 0x8, 0x68, 0x99, 0x15, 0x30, 0x91, 0x64, 0x10, 0x10,
    ];
    assert_eq!(
        MSInfoChangeNotificationRequest::unmarshal(encoded),
        Err(GTPV1Error::MessageMandatoryIEMissing)
    );
}
