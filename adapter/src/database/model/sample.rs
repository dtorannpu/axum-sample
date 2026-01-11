use kernel::model::id::SampleId;
use kernel::model::sample::Sample;

pub(crate) struct SampleRow {
    pub id: SampleId,
    pub name: String,
    pub email: String,
    pub age: i16,
}

impl From<SampleRow> for Sample {
    fn from(value: SampleRow) -> Self {
        let SampleRow {
            id,
            name,
            email,
            age,
        } = value;
        Sample {
            id,
            name,
            email,
            age,
        }
    }
}
