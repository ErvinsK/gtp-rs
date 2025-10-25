// MM Contex IE  - according to 3GPP TS 29.274 V17.10.0 (2023-12)

use crate::gtpv2::{errors::GTPV2Error, messages::ies::*};

// Enum for MM Context IE type

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MmContext {
    MmContextEpsSecurityContextQuadruplets(Box<MmContextEpsSecurityContextQuadruplets>),
    MmContextUmtsKeyQuintuplets(MmContextUmtsKeyQuintuplets),
    MmContextUmtsKeyQuadrupletsQuintuplets(MmContextUmtsKeyQuadrupletsQuintuplets),
    MmContextUmtsKeyCipherQuintuplets(MmContextUmtsKeyCipherQuintuplets),
    MmContextGsmKeyTriplets(MmContextGsmKeyTriplets),
    MmContextGsmKeyCipherQuintuplets(MmContextGsmKeyCipherQuintuplets),
}

impl Default for MmContext {
    fn default() -> MmContext {
        MmContext::MmContextEpsSecurityContextQuadruplets(Box::default())
    }
}

impl From<MmContextEpsSecurityContextQuadruplets> for MmContext {
    fn from(i: MmContextEpsSecurityContextQuadruplets) -> Self {
        MmContext::MmContextEpsSecurityContextQuadruplets(Box::new(i))
    }
}

impl From<MmContextUmtsKeyQuintuplets> for MmContext {
    fn from(i: MmContextUmtsKeyQuintuplets) -> Self {
        MmContext::MmContextUmtsKeyQuintuplets(i)
    }
}

impl From<MmContextUmtsKeyQuadrupletsQuintuplets> for MmContext {
    fn from(i: MmContextUmtsKeyQuadrupletsQuintuplets) -> Self {
        MmContext::MmContextUmtsKeyQuadrupletsQuintuplets(i)
    }
}

impl From<MmContextUmtsKeyCipherQuintuplets> for MmContext {
    fn from(i: MmContextUmtsKeyCipherQuintuplets) -> Self {
        MmContext::MmContextUmtsKeyCipherQuintuplets(i)
    }
}

impl From<MmContextGsmKeyTriplets> for MmContext {
    fn from(i: MmContextGsmKeyTriplets) -> Self {
        MmContext::MmContextGsmKeyTriplets(i)
    }
}

impl From<MmContextGsmKeyCipherQuintuplets> for MmContext {
    fn from(i: MmContextGsmKeyCipherQuintuplets) -> Self {
        MmContext::MmContextGsmKeyCipherQuintuplets(i)
    }
}

impl From<MmContext> for InformationElement {
    fn from(i: MmContext) -> Self {
        InformationElement::MmContext(i)
    }
}

impl MmContext {
    pub fn get_ins(&self) -> u8 {
        match self {
            MmContext::MmContextEpsSecurityContextQuadruplets(i) => i.ins,
            MmContext::MmContextUmtsKeyQuintuplets(i) => i.ins,
            MmContext::MmContextUmtsKeyQuadrupletsQuintuplets(i) => i.ins,
            MmContext::MmContextUmtsKeyCipherQuintuplets(i) => i.ins,
            MmContext::MmContextGsmKeyTriplets(i) => i.ins,
            MmContext::MmContextGsmKeyCipherQuintuplets(i) => i.ins,
        }
    }
}

impl IEs for MmContext {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        match self {
            MmContext::MmContextEpsSecurityContextQuadruplets(i) => i.marshal(buffer),
            MmContext::MmContextUmtsKeyQuintuplets(i) => i.marshal(buffer),
            MmContext::MmContextUmtsKeyQuadrupletsQuintuplets(i) => i.marshal(buffer),
            MmContext::MmContextUmtsKeyCipherQuintuplets(i) => i.marshal(buffer),
            MmContext::MmContextGsmKeyTriplets(i) => i.marshal(buffer),
            MmContext::MmContextGsmKeyCipherQuintuplets(i) => i.marshal(buffer),
        }
    }

    fn unmarshal(buffer: &[u8]) -> Result<Self, GTPV2Error> {
        match buffer[0] {
            MMCTXEPSSECCTXQ => match MmContextEpsSecurityContextQuadruplets::unmarshal(buffer) {
                Ok(i) => Ok(MmContext::MmContextEpsSecurityContextQuadruplets(Box::new(
                    i,
                ))),
                Err(e) => Err(e),
            },
            MMCTXUMTSKQ => match MmContextUmtsKeyQuintuplets::unmarshal(buffer) {
                Ok(i) => Ok(MmContext::MmContextUmtsKeyQuintuplets(i)),
                Err(e) => Err(e),
            },
            MMCTXUMTSKQQ => match MmContextUmtsKeyQuadrupletsQuintuplets::unmarshal(buffer) {
                Ok(i) => Ok(MmContext::MmContextUmtsKeyQuadrupletsQuintuplets(i)),
                Err(e) => Err(e),
            },
            MMCTXUMTSKCQ => match MmContextUmtsKeyCipherQuintuplets::unmarshal(buffer) {
                Ok(i) => Ok(MmContext::MmContextUmtsKeyCipherQuintuplets(i)),
                Err(e) => Err(e),
            },
            MMCTXGSMKT => match MmContextGsmKeyTriplets::unmarshal(buffer) {
                Ok(i) => Ok(MmContext::MmContextGsmKeyTriplets(i)),
                Err(e) => Err(e),
            },
            MMCTXGSMKCQ => match MmContextGsmKeyCipherQuintuplets::unmarshal(buffer) {
                Ok(i) => Ok(MmContext::MmContextGsmKeyCipherQuintuplets(i)),
                Err(e) => Err(e),
            },
            _ => Err(GTPV2Error::IEIncorrect(buffer[0])),
        }
    }

    fn len(&self) -> usize {
        match self {
            MmContext::MmContextEpsSecurityContextQuadruplets(i) => i.len(),
            MmContext::MmContextUmtsKeyQuintuplets(i) => i.len(),
            MmContext::MmContextUmtsKeyQuadrupletsQuintuplets(i) => i.len(),
            MmContext::MmContextUmtsKeyCipherQuintuplets(i) => i.len(),
            MmContext::MmContextGsmKeyTriplets(i) => i.len(),
            MmContext::MmContextGsmKeyCipherQuintuplets(i) => i.len(),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            MmContext::MmContextEpsSecurityContextQuadruplets(i) => i.is_empty(),
            MmContext::MmContextUmtsKeyQuintuplets(i) => i.is_empty(),
            MmContext::MmContextUmtsKeyQuadrupletsQuintuplets(i) => i.is_empty(),
            MmContext::MmContextUmtsKeyCipherQuintuplets(i) => i.is_empty(),
            MmContext::MmContextGsmKeyTriplets(i) => i.is_empty(),
            MmContext::MmContextGsmKeyCipherQuintuplets(i) => i.is_empty(),
        }
    }

    fn get_ins(&self) -> u8 {
        match self {
            MmContext::MmContextEpsSecurityContextQuadruplets(i) => i.ins,
            MmContext::MmContextUmtsKeyQuintuplets(i) => i.ins,
            MmContext::MmContextUmtsKeyQuadrupletsQuintuplets(i) => i.ins,
            MmContext::MmContextUmtsKeyCipherQuintuplets(i) => i.ins,
            MmContext::MmContextGsmKeyTriplets(i) => i.ins,
            MmContext::MmContextGsmKeyCipherQuintuplets(i) => i.ins,
        }
    }

    fn get_type(&self) -> u8 {
        match self {
            MmContext::MmContextEpsSecurityContextQuadruplets(_) => MMCTXEPSSECCTXQ,
            MmContext::MmContextUmtsKeyQuintuplets(_) => MMCTXUMTSKQ,
            MmContext::MmContextUmtsKeyQuadrupletsQuintuplets(_) => MMCTXUMTSKQQ,
            MmContext::MmContextUmtsKeyCipherQuintuplets(_) => MMCTXUMTSKCQ,
            MmContext::MmContextGsmKeyTriplets(_) => MMCTXGSMKT,
            MmContext::MmContextGsmKeyCipherQuintuplets(_) => MMCTXGSMKCQ,
        }
    }
}
