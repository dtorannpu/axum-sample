use garde::Validate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SampleRequest {
    #[garde(length(min = 1, max = 30))]
    pub name: String,
    #[garde(range(min = 0, max = 100))]
    pub age: u8,
}
