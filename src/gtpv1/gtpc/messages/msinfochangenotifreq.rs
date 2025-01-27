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


