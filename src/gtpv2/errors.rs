use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]

pub enum GTPV2Error {
    // GTPv2 Header Errors
    HeaderInvalidLength,
    HeaderVersionNotSupported,
    HeaderFlagError,
    HeaderTypeMismatch,
    MandatoryHeaderFlagError,
    // GTPv2 IE Errors
    IEInvalidLength(u8),
    IEIncorrect(u8),
    // GTPv2 Message Errors
    MessageInvalidLength(u8),
    MessageNotSupported,
    MessageMandatoryIEMissing(u8),
    MessageOptionalIEIncorrect(u8),
    MessageInvalidMessageFormat,
    MessageIncorrectMessageType,
}

impl std::error::Error for GTPV2Error {}

impl Display for GTPV2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // GTPv2 Header Errors
            GTPV2Error::HeaderInvalidLength => write!(f, "Invalid Header length"),
            GTPV2Error::HeaderVersionNotSupported => write!(f, "GTP Version not supported"),
            GTPV2Error::HeaderFlagError => write!(f, "Header flag error"),
            GTPV2Error::HeaderTypeMismatch => write!(f, "Header type mismatch"),
            GTPV2Error::MandatoryHeaderFlagError => write!(
                f,
                "Mandatory header flag is not properly set for the particular GTP message"
            ),
            // GTPv2 IE Errors
            GTPV2Error::IEInvalidLength(i) => write!(f, "Invalid IE type {} length", i),
            GTPV2Error::IEIncorrect(i) => write!(f, "Incorrect IE type {}", i),
            // GTPv2 Message Errors
            GTPV2Error::MessageInvalidLength(i) => write!(f, "Message invalid length {}", i),
            GTPV2Error::MessageMandatoryIEMissing(i) => {
                write!(f, "Mandatory IE of type {} missing", i)
            }
            GTPV2Error::MessageNotSupported => write!(f, "Message not supported"),
            GTPV2Error::MessageOptionalIEIncorrect(i) => {
                write!(f, "Optional IE of type {} incorrect", i)
            }
            GTPV2Error::MessageInvalidMessageFormat => write!(f, "Invalid Mesage format"),
            GTPV2Error::MessageIncorrectMessageType => write!(f, "Incorrect Message type"),
        }
    }
}
