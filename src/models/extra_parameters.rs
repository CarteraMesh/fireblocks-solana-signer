use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtraParameters {
    #[serde(rename = "programCallData")]
    pub program_call_data: String,
}

impl ExtraParameters {
    pub fn new(program_call_data: String) -> Self {
        Self { program_call_data }
    }
}
