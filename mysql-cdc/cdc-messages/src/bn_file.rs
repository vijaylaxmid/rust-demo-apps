use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BnFile {
    pub file_name: String,
    pub offset: Option<u64>,
}

impl BnFile {
    #![allow(dead_code)] // used in unit tests
    pub fn new(file_name: String, offset: Option<u64>) -> Self {
        Self { file_name, offset }
    }
}
