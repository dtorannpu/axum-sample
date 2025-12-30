use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SampleId(i64);

impl SampleId {
    pub fn new(id: i64) -> Self {
        SampleId(id)
    }

    pub fn get(&self) -> i64 {
        self.0
    }
}

impl From<i64> for SampleId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
