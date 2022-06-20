use std::fmt::Display;

#[derive(Debug)]

pub enum GTPUError {
    HeaderSizeTooSmall,
    HeaderSizeMismatch,
    HeaderFlagError,
    HeaderTypeMismatch,
    MandatoryHeaderFlagError,
    IETypeMismatch,
    InvalidIELength,
    MessageLengthError,
    MessageNotSupported,
    MandatoryIEMissing,
}

impl std::error::Error for GTPUError{}

impl Display for GTPUError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GTPUError::HeaderSizeTooSmall => write!(f, "Header too small"),
            GTPUError::HeaderSizeMismatch => write!(f, "Header size mismatch"),
            GTPUError::HeaderFlagError => write!(f, "Header flag error"),
            GTPUError::HeaderTypeMismatch => write!(f, "Header type mismatch"),
            GTPUError::MandatoryHeaderFlagError => write!(f, "Mandatory header flag is not properly set for the particular GTP message"),
            GTPUError::IETypeMismatch => write!(f, "IE type mismatch"),
            GTPUError::InvalidIELength => write!(f, "Invalid IE length"),
            GTPUError::MessageLengthError => write!(f, "Message length error"),
            GTPUError::MandatoryIEMissing => write!(f, "Mandatory IE missing"),
            GTPUError::MessageNotSupported => write!(f, "Message not supported"),
        }
    }
}