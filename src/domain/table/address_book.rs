use crate::domain::dto::address_book_dto::AddressBookPageDTO;

use rbatis::rbdc::DateTime;
use rbatis::{crud, html_sql};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AddressBook {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub consignee: Option<String>,
    pub sex: Option<i32>,
    pub phone: Option<String>,
    pub province_code: Option<String>,
    pub province_name: Option<String>,
    pub city_code: Option<String>,
    pub city_name: Option<String>,
    pub district_code: Option<String>,
    pub district_name: Option<String>,
    pub detail: Option<String>,
    pub label: Option<String>,
    pub is_default: Option<i32>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub create_user: Option<String>,
    pub update_user: Option<String>,
    pub is_deleted: Option<i32>,
}

crud!(AddressBook {});

#[html_sql("src/domain/table/address_book.html")]
impl AddressBook {
    pub async fn select_page(conn: &dyn rbatis::Executor, page_request: &rbatis::PageRequest, dto: &AddressBookPageDTO) -> rbatis::Result<rbatis::Page<AddressBook>> { impled!() }
}
