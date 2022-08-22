use std::fmt::Display;

#[derive(Debug, PartialEq)]

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
    MessageLengthError,
    MessageNotSupported,
    MessageMandatoryIEMissing,
    MessageOptionalIEIncorrect,
    MessageInvalidMessageFormat,
    MessageIncorrectMessageType,
}

impl std::error::Error for GTPV2Error{}

impl Display for GTPV2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
// GTPv2 Header Errors
            GTPV2Error::HeaderInvalidLength => write!(f, "Invalid Header lenght"),
            GTPV2Error::HeaderVersionNotSupported => write!(f, "GTP Version not supported"),
            GTPV2Error::HeaderFlagError => write!(f, "Header flag error"),
            GTPV2Error::HeaderTypeMismatch => write!(f, "Header type mismatch"),
            GTPV2Error::MandatoryHeaderFlagError => write!(f, "Mandatory header flag is not properly set for the particular GTP message"),
// GTPv2 IE Errors
            GTPV2Error::IEInvalidLength(i) => write!(f, "Invalid IE type {} length", i),
            GTPV2Error::IEIncorrect(i)  => write!(f, "Incorrect IE type {}", i),
// GTPv2 Message Errors
            GTPV2Error::MessageLengthError => write!(f, "Message length error"),
            GTPV2Error::MessageMandatoryIEMissing => write!(f, "Mandatory IE missing"),
            GTPV2Error::MessageNotSupported => write!(f, "Message not supported"),
            GTPV2Error::MessageOptionalIEIncorrect => write!(f, "Optional IE incorrect"),
            GTPV2Error::MessageInvalidMessageFormat => write!(f, "Invalid Mesage format"),
            GTPV2Error::MessageIncorrectMessageType => write!(f, "Incorrect Message type"),
        }
    }
}