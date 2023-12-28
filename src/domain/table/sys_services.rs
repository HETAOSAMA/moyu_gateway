use rbatis::{crud, impl_select};
use rbatis::rbdc::DateTime;

#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SysServices {
    pub id: Option<String>,
    pub server_name: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub protocol: Option<String>,
    pub port: Option<i32>,
    pub path: Option<String>,
    pub is_active: Option<i32>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}
crud!(SysServices{});
impl_select!(SysServices{select_by_id(id:&String) -> Option => "`where id = #{id}`"});
