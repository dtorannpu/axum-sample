use garde::Validate;
use kernel::model::id::SampleId;
use kernel::model::sample::Sample;
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
    pub id: SampleId,
    pub name: String,
    pub email: String,
    pub age: i32,
}

impl From<Sample> for SampleResponse {
    fn from(value: Sample) -> Self {
        let Sample {
            id,
            name,
            email,
            age,
        } = value;

        Self {
            id,
            name,
            email,
            age,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SampleList {
    pub samples: Vec<SampleResponse>,
}

impl From<Vec<Sample>> for SampleList {
    fn from(value: Vec<Sample>) -> Self {
        Self {
            samples: value.into_iter().map(SampleResponse::from).collect(),
        }
    }
}
