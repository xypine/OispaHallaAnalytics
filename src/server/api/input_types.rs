use poem_openapi::Object;

#[derive(Object)]
pub struct RecordInput {
    pub r: String,
    pub abandoned: bool,
    pub won: bool,
    pub score: usize
}