use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]

pub enum GTPV1Error {
    // GTPv1 Header Errors
    HeaderInvalidLength,
    HeaderVersionNotSupported,
    HeaderFlagError,
    HeaderTypeMismatch,
    MandatoryHeaderFlagError,
    // GTPv1 Extension Header Errors
    ExtHeaderInvalidLength,
    ExtHeaderUnknown,
    // GTPv1 IE Errors
    IETypeMismatch,
    IEInvalidLength,
    IEIncorrect,
    // GTPv1 Message Errors
    MessageLengthError,
    MessageNotSupported,
    MessageMandatoryIEMissing,
    MessageOptionalIEIncorrect,
    MessageInvalidMessageFormat,
    MessageIncorrectMessageType,
}

impl std::error::Error for GTPV1Error {}

impl Display for GTPV1Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // GTPv1 Header Errors
            GTPV1Error::HeaderInvalidLength => write!(f, "Invalid Header length"),
            GTPV1Error::HeaderVersionNotSupported => write!(f, "GTP Version not supported"),
            GTPV1Error::HeaderFlagError => write!(f, "Header flag error"),
            GTPV1Error::HeaderTypeMismatch => write!(f, "Header type mismatch"),
            GTPV1Error::MandatoryHeaderFlagError => write!(
                f,
                "Mandatory header flag is not properly set for the particular GTP message"
            ),
            // GTPv1 Extension Header Errors
            GTPV1Error::ExtHeaderInvalidLength => write!(f, "Invalid Extension Header length"),
            GTPV1Error::ExtHeaderUnknown => write!(f, "Incorrect Extension Header"),
            // GTPv1 IE Errors
            GTPV1Error::IETypeMismatch => write!(f, "IE type mismatch"),
            GTPV1Error::IEInvalidLength => write!(f, "Invalid IE length"),
            GTPV1Error::IEIncorrect => write!(f, "Incorrect IE"),
            // GTPv1 Message Errors
            GTPV1Error::MessageLengthError => write!(f, "Message length error"),
            GTPV1Error::MessageMandatoryIEMissing => write!(f, "Mandatory IE missing"),
            GTPV1Error::MessageNotSupported => write!(f, "Message not supported"),
            GTPV1Error::MessageOptionalIEIncorrect => write!(f, "Optional IE incorrect"),
            GTPV1Error::MessageInvalidMessageFormat => write!(f, "Invalid Mesage format"),
            GTPV1Error::MessageIncorrectMessageType => write!(f, "Incorrect Message type"),
        }
    }
}
