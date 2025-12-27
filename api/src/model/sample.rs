use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SampleRequest {
    #[garde(length(utf16, min = 1, max = 100))]
    pub name: String,
    #[garde(range(min = 0, max = 100))]
    pub age: u8,
}

#[derive(Debug, Serialize)]
pub struct SampleResponse {
    pub name: String,
    pub age: u8,
}

#[derive(Debug, Serialize)]
pub struct SampleList {
    pub samples: Vec<SampleResponse>,
}
