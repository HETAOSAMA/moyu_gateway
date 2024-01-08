use rbatis::{crud, impl_select};
use rbatis::rbdc::DateTime;
use rbatis::table_sync::{
    ColumMapper, MssqlTableMapper, MysqlTableMapper, PGTableMapper, SqliteTableMapper,
};
use rbatis::RBatis;
use crate::service::redis_service::{get, set};
use crate::service::sys_services_service::get_redis_key;

#[derive(Clone, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SysServices {
    pub id: Option<String>,
    pub server_name: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub protocol: Option<String>,
    pub port: Option<u16>,
    pub path: Option<String>,
    pub is_active: Option<i32>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}
crud!(SysServices{});
impl_select!(SysServices{select_by_id(id:&String) -> Option => "`where id = #{id}`"});


//将数据库中的数据同步到redis中
pub async fn sync_tables_data(rb: &RBatis) {
    let mut sys_services = SysServices::select_by_column(rb, "is_active",1).await.unwrap();
    if sys_services.is_empty(){
        return;
    }
    for sys_service in sys_services {
        let key = get_redis_key(sys_service.clone());
        if key.clone().unwrap().is_empty() {
            continue;
        }
        let _ = set(key.clone().unwrap().to_string(), serde_json::to_string(&sys_service).unwrap()).await.await;
    }
}

