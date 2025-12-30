use garde::Validate;
use kernel::model::id::SampleId;
use kernel::model::sample::Sample;
use kernel::model::sample::event::CreateSample;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SampleResponse {
    pub id: SampleId,
    pub name: String,
    pub email: String,
    pub age: i16,
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

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateSampleRequest {
    #[garde(length(utf16, max = 100))]
    pub name: String,
    #[garde(email)]
    pub email: String,
    #[garde(range(min = 0, max = 100))]
    pub age: i16,
}

impl From<CreateSampleRequest> for CreateSample {
    fn from(value: CreateSampleRequest) -> Self {
        let CreateSampleRequest { name, email, age } = value;

        Self { name, email, age }
    }
}
