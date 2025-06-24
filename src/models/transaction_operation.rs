use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionOperation {
    #[serde(rename = "PROGRAM_CALL")]
    ProgramCall,
}

impl std::fmt::Display for TransactionOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ProgramCall => write!(f, "PROGRAM_CALL"),
        }
    }
}

impl Default for TransactionOperation {
    fn default() -> TransactionOperation {
        Self::ProgramCall
    }
}
