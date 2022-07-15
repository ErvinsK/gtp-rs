// Recovery IE - according to 3GPP TS 29.060 V15.5.0 (2019-06)

use crate::gtpv1::gtpc::ies::commons::{*};

// Recovery IE TV 

pub const RECOVERY_LENGTH:usize = 1;
pub const RECOVERY:u8 = 14;

// Recovery IE implementation

#[derive(Debug, Clone)]
pub struct Recovery {
    pub t:u8,
    pub restart_counter:u8,
}

impl Default for Recovery {
    fn default() -> Recovery {
        Recovery {
            t:RECOVERY,
            restart_counter:0,
        }
    }
}

impl IEs for Recovery {
    fn marshal(&self, buffer: &mut Vec<u8>) {
        buffer.push(RECOVERY);
        buffer.push(self.restart_counter);
    }

    fn unmarshal(buffer: &[u8]) -> Option<Recovery> {
        if buffer.len()>=RECOVERY_LENGTH {
            Some(Recovery { t:RECOVERY, restart_counter: buffer[1] })
        } else { 
            None
        }
        
    }

    fn len(&self) -> usize {
        RECOVERY_LENGTH+1
    }
}
