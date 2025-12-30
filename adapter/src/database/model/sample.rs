use kernel::model::id::SampleId;
use kernel::model::sample::Sample;
use shared::error::AppError;

pub(crate) struct SampleRow {
    pub id: SampleId,
    pub name: String,
    pub age: i32,
}

impl TryFrom<SampleRow> for Sample {
    type Error = AppError;

    fn try_from(value: SampleRow) -> Result<Self, Self::Error> {
        let SampleRow { id, name, age } = value;
        Ok(Sample { id, name, age })
    }
}
