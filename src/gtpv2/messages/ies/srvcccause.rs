// SRVCC Cause IE - according to 3GPP TS 29.280 V17.0.0 (2022-04)

use crate::gtpv2::{
    errors::GTPV2Error,
    messages::ies::{commons::*, ie::*},
    utils::*,
};

// SRVCC Cause IE Type

pub const SRVCC_CAUSE: u8 = 56;
pub const SRVCC_CAUSE_LENGTH: usize = 1;

/* SRVCC Cause Enum

Cause value (decimal)           	Meaning

0	                                Reserved. Shall not be sent and if received the Cause shall be treated as an invalid IE
1	                                Unspecified
2	                                Handover/Relocation cancelled by source system
3	                                Handover /Relocation Failure with Target system
4                               	Handover/Relocation Target not allowed
5	                                Unknown Target ID
6	                                Target Cell not available
7	                                No Radio Resources Available in Target Cell
8	                                Failure in Radio Interface Procedure
9	                                Permanent session leg establishment error
10                              	Temporary session leg establishment error
11-255	                            Spare. This value range is reserved for SRVCC Cause values
*/

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SrvccCauseValues {
    Reserved,
    #[default]
    Unspecified,
    HandoverRelocationCancelledBySourceSystem,
    HandoverRelocationFailureWithTargetSystem,
    HandoverRelocationTargetNotAllowed,
    UnknownTargetId,
    TargetCellNotAvailable,
    NoRadioResourcesAvailableInTargetCell,
    FailureInRadioInterfaceProcedure,
    PermanentSessionLegEstablishmentError,
    TemporarySessionLegEstablishmentError,
    Spare,
}

impl From<&SrvccCauseValues> for u8 {
    fn from(i: &SrvccCauseValues) -> u8 {
        match i {
            SrvccCauseValues::Reserved => 0,
            SrvccCauseValues::Unspecified => 1,
            SrvccCauseValues::HandoverRelocationCancelledBySourceSystem => 2,
            SrvccCauseValues::HandoverRelocationFailureWithTargetSystem => 3,
            SrvccCauseValues::HandoverRelocationTargetNotAllowed => 4,
            SrvccCauseValues::UnknownTargetId => 5,
            SrvccCauseValues::TargetCellNotAvailable => 6,
            SrvccCauseValues::NoRadioResourcesAvailableInTargetCell => 7,
            SrvccCauseValues::FailureInRadioInterfaceProcedure => 8,
            SrvccCauseValues::PermanentSessionLegEstablishmentError => 9,
            SrvccCauseValues::TemporarySessionLegEstablishmentError => 10,
            SrvccCauseValues::Spare => 11,
        }
    }
}

impl From<u8> for SrvccCauseValues {
    fn from(i: u8) -> Self {
        match i {
            0 => SrvccCauseValues::Reserved,
            1 => SrvccCauseValues::Unspecified,
            2 => SrvccCauseValues::HandoverRelocationCancelledBySourceSystem,
            3 => SrvccCauseValues::HandoverRelocationFailureWithTargetSystem,
            4 => SrvccCauseValues::HandoverRelocationTargetNotAllowed,
            5 => SrvccCauseValues::UnknownTargetId,
            6 => SrvccCauseValues::TargetCellNotAvailable,
            7 => SrvccCauseValues::NoRadioResourcesAvailableInTargetCell,
            8 => SrvccCauseValues::FailureInRadioInterfaceProcedure,
            9 => SrvccCauseValues::PermanentSessionLegEstablishmentError,
            10 => SrvccCauseValues::TemporarySessionLegEstablishmentError,
            _ => SrvccCauseValues::Spare,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SrvccCause {
    pub t: u8,
    pub length: u16,
    pub ins: u8,
    pub value: SrvccCauseValues,
}

impl Default for SrvccCause {
    fn default() -> Self {
        SrvccCause {
            t: SRVCC_CAUSE,
            length: SRVCC_CAUSE_LENGTH as u16,
            ins: 0,
            value: SrvccCauseValues::default(),
        }
    }
}

impl From<SrvccCause> for InformationElement {
    fn from(i: SrvccCause) -> Self {
        InformationElement::SrvccCause(i)
    }
}

impl IEs for SrvccCause {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie: Vec<u8> = vec![];
        buffer_ie.push(SRVCC_CAUSE);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(u8::from(&self.value));
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len() >= MIN_IE_SIZE + SRVCC_CAUSE_LENGTH {
            let data = SrvccCause {
                length: u16::from_be_bytes([buffer[1], buffer[2]]),
                ins: buffer[3] & 0x0f,
                value: buffer[4].into(),
                ..SrvccCause::default()
            };
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(SRVCC_CAUSE))
        }
    }

    fn len(&self) -> usize {
        SRVCC_CAUSE_LENGTH + MIN_IE_SIZE
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }
    fn get_ins(&self) -> u8 {
        self.ins
    }
    fn get_type(&self) -> u8 {
        self.t
    }
}

#[test]
fn srvcc_cause_ie_marshal_test() {
    let encoded: [u8; 5] = [0x38, 0x00, 0x01, 0x00, 0x05];
    let decoded = SrvccCause {
        t: SRVCC_CAUSE,
        length: SRVCC_CAUSE_LENGTH as u16,
        ins: 0,
        value: SrvccCauseValues::UnknownTargetId,
    };
    let mut buffer: Vec<u8> = vec![];
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}

#[test]
fn srvcc_cause_ie_unmarshal_test() {
    let encoded: [u8; 5] = [0x38, 0x00, 0x01, 0x00, 0x05];
    let decoded = SrvccCause {
        t: SRVCC_CAUSE,
        length: SRVCC_CAUSE_LENGTH as u16,
        ins: 0,
        value: SrvccCauseValues::UnknownTargetId,
    };
    assert_eq!(SrvccCause::unmarshal(&encoded).unwrap(), decoded);
}
