use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetParams {
    pub id: String,
}
