use poem_openapi::Object;

#[derive(Object)]
pub struct RecordInput {
    pub r: String,
    pub client: String,
}

#[derive(Object)]
pub struct WipeInput {
    pub key: String,
}
