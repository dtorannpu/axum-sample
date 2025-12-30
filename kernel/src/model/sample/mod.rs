pub mod event;

use crate::model::id::SampleId;

#[derive(Debug, PartialEq, Eq)]
pub struct Sample {
    pub id: SampleId,
    pub name: String,
    pub email: String,
    pub age: i16,
}
