use rbatis::rbdc::DateTime;
use rbatis::snowflake::new_snowflake_id;
use crate::domain::table::sys_services::SysServices;
use crate::pool;
use crate::error::Result;
use crate::service::redis_service::set;
use crate::error::Error;

pub struct SysServiceService {}

impl SysServiceService {
    pub async fn add_service(&self, mut arg: SysServices) -> Result<u64> {
        arg.id = Option::from(new_snowflake_id().to_string());

        let current_datetime = DateTime::now();
        arg.created_at = Some(current_datetime.clone());
        arg.updated_at = Some(current_datetime);
        match serde_json::to_string(&arg) {
            Ok(json_string) => {
                set(arg.id.clone().unwrap(), json_string);
            }
            Err(e) => {
                return Err(Error::from(e.to_string()));
            }
        }
        let result = Ok(SysServices::insert(pool!(), &arg).await?.rows_affected);
        result
    }

    pub async fn update_service(&self, mut arg: SysServices) -> Result<u64> {
        let current_datetime = DateTime::now();
        arg.updated_at = Some(current_datetime);
        let flag = SysServices::update_by_column(pool!(), &arg, "id").await?.rows_affected;
        let data = SysServices::select_by_id(pool!(), &arg.id.clone().unwrap()).await?.unwrap();
        match serde_json::to_string(&data) {
            Ok(json_string) => {
                set(data.id.clone().unwrap(), json_string);
            }
            Err(e) => {
                return Err(Error::from(e.to_string()));
            }
        }
        let result = Ok(flag);
        result
    }
}