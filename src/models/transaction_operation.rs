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

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_tx_op() {
        assert_eq!(
            TransactionOperation::default(),
            TransactionOperation::ProgramCall
        );
        let op = format!("{}", TransactionOperation::default());
        assert_eq!("PROGRAM_CALL", op);
    }
}
