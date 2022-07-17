use std::fmt::Display;

#[derive(Debug, PartialEq)]

pub enum GTPV1Error {
    HeaderSizeTooSmall,
    HeaderSizeMismatch,
    HeaderFlagError,
    HeaderTypeMismatch,
    MandatoryHeaderFlagError,
    IETypeMismatch,
    InvalidIELength,
    IncorrectIE,
    MessageLengthError,
    MessageNotSupported,
    MandatoryIEMissing,
}

impl std::error::Error for GTPV1Error{}

impl Display for GTPV1Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
// GTPv1 Header Errors
            GTPV1Error::HeaderSizeTooSmall => write!(f, "Header too small"),
            GTPV1Error::HeaderSizeMismatch => write!(f, "Header size mismatch"),
            GTPV1Error::HeaderFlagError => write!(f, "Header flag error"),
            GTPV1Error::HeaderTypeMismatch => write!(f, "Header type mismatch"),
            GTPV1Error::MandatoryHeaderFlagError => write!(f, "Mandatory header flag is not properly set for the particular GTP message"),
// GTPv1 IE Errors
            GTPV1Error::IETypeMismatch => write!(f, "IE type mismatch"),
            GTPV1Error::InvalidIELength => write!(f, "Invalid IE length"),
            GTPV1Error::IncorrectIE => write!(f, "Incorrect IE"),
// GTPv1 Message Errors
            GTPV1Error::MessageLengthError => write!(f, "Message length error"),
            GTPV1Error::MandatoryIEMissing => write!(f, "Mandatory IE missing"),
            GTPV1Error::MessageNotSupported => write!(f, "Message not supported"),
        }
    }
}