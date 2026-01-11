use crate::model::id::SampleId;

pub struct CreateSample {
    pub name: String,
    pub email: String,
    pub age: i16,
}

pub struct UpdateSample {
    pub id: SampleId,
    pub name: String,
    pub email: String,
    pub age: i16,
}

pub struct DeleteSample {
    pub id: SampleId,
}
