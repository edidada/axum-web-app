use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DictPageDTO {
    pub code: Option<String>,
    pub name: Option<String>,
    // pub limit: Option<u64>,
    // pub offset: Option<u64>,
}
