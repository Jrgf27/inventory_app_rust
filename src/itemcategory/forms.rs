use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ItemCategoryForm {
    pub authenticity_token: String,
    pub id: i64,
    pub name: String,
    pub description: String,
}
