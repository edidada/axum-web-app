use crate::domain::dto::user_dto::UserPageDTO;

use rbatis::{crud, html_sql};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub sex: Option<i32>,
    pub id_number: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<i32>,
}

crud!(User {});

#[html_sql("src/domain/table/user.html")]
impl User {
    pub async fn select_page(
        conn: &dyn rbatis::Executor,
        page_request: &rbatis::PageRequest,
        dto: &UserPageDTO,
    ) -> rbatis::Result<rbatis::Page<User>> {
        impled!()
    }
}
