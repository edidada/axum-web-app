use crate::domain::dto::dict_dto::DictPageDTO;

use rbatis::rbdc::DateTime;
use rbatis::{crud, html_sql};
use serde::ser::Serializer;

fn serialize_datetime<S>(date: &Option<DateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(d) => {
            // 格式化为 yyyyMMdd hhmmss
            let formatted = format!(
                "{}{:02}{:02} {:02}{:02}{:02}",
                d.year(),
                d.mon(),
                d.day(),
                d.hour(),
                d.minute(),
                d.sec()
            );
            serializer.serialize_str(&formatted)
        }
        None => serializer.serialize_none(),
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Hash, Eq, PartialEq)]
pub struct SysDict {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub value: Option<String>,
    pub sort: Option<i32>,
    pub remark: Option<String>,
    pub state: Option<i32>,
    #[serde(serialize_with = "serialize_datetime")]
    pub create_date: Option<DateTime>,
}

crud!(SysDict {});

#[html_sql("src/domain/table/sys_dict.html")]
impl SysDict {
    pub async fn select_page(
        conn: &dyn rbatis::Executor,
        request: &rbatis::PageRequest,
        dto: &DictPageDTO,
    ) -> rbatis::Result<rbatis::Page<SysDict>> {
        impled!()
    }
}
