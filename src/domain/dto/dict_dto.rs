use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DictPageDTO {
    pub code: Option<String>,
    pub name: Option<String>,
}