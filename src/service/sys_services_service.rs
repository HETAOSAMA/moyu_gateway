use rbatis::{html_sql, RBatis};
use rbatis::rbdc::DateTime;
use rbatis::rbdc::db::ExecResult;
use rbatis::snowflake::new_snowflake_id;
use rbs::Value;
use rbs::Value::Null;
use serde_json::from_str;
use url::{Url};
use crate::domain::table::sys_services::SysServices;
use crate::pool;
use crate::error::Result;
use crate::service::redis_service::{del, get, set};
use crate::error::Error;

#[html_sql("src/domain/table/sys_services.html")]
pub async fn delete_by_ids(rb: &RBatis,arg: Vec<String>) -> ExecResult {
    impled!()
}
#[html_sql("src/domain/table/sys_services.html")]
pub async fn select_by_ids(rb: &RBatis,arg: Vec<String>) -> Vec<SysServices> {
    impled!()
}

pub struct SysServiceService {}

impl SysServiceService {
    pub async fn add_service(&self, mut arg: SysServices) -> Result<u64> {
        let current_datetime = DateTime::now();

        // 添加默认协议 防止写为 http://http://
        let mut url = arg.url.clone().unwrap();
        let arg_url_with_scheme = if url.starts_with("http://") || url.starts_with("https://") {
            arg.url.unwrap().to_string()
        } else {
            format!("http://{}", arg.url.unwrap())
        };
        // 解析 URL
        let mut parsed_url = Url::parse(&arg_url_with_scheme);
        match parsed_url {
            Ok(_) => {
                arg.url = Some(parsed_url.unwrap().to_string());
            }
            Err(_) => {
                return Err(Error::from("url 解析失败"));
            }
        }

        arg.id = Option::from(new_snowflake_id().to_string());
        arg.created_at = Some(current_datetime.clone());
        arg.updated_at = Some(current_datetime);
        match serde_json::to_string(&arg) {
            Ok(json_string) => {
                let key = get_redis_key(arg.clone());
                if key.is_null() {
                    return Err(Error::from("redis key 生成失败"));
                }
                //查看redis中是否存在
                let value = get(key.to_string());
                if value != "" {
                    return Err(Error::from("服务已存在"));
                }
                set(key.to_string(), json_string);
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
                let key = get_redis_key(arg.clone());
                if key.is_null() {
                    return Err(Error::from("redis key 生成失败"));
                }
                //查看redis中是否存在
                let json_str = get(key.to_string());
                if json_str != "" {
                    match from_str::<SysServices>(json_str.as_str()) {
                        Ok(result) => {
                            if result.id != arg.id {
                                return Err(Error::from("已有同名服务名称和路径,请重新命名"));
                            }else {
                                set(key.to_string(), json_string);
                            }
                        }
                        Err(err) => {
                            return Err(Error::from("反序列化失败"));
                        }
                    }
                }
            }
            Err(e) => {
                return Err(Error::from(e.to_string()));
            }
        }
        let result = Ok(flag);
        result
    }

    pub async fn delete_service_by_ids(&self, arg: Vec<String>) -> Result<u64> {
        let datas = select_by_ids(pool!(), arg.clone()).await?;
        if !datas.is_empty(){
            for data in datas {
                let redis_key = get_redis_key(data).to_string();
                del(redis_key);
            }
        }
        let a = delete_by_ids(pool!(), arg).await?.rows_affected;
        Ok(a)
    }
}


//生成redis key
fn get_redis_key(mut arg: SysServices) -> Value {
    let mut redis_key = Null;
    if arg.server_name.is_some() {
        redis_key = Value::String(arg.server_name.unwrap());
    }
    if arg.path.is_some() {
        redis_key = Value::String(redis_key.to_string()+arg.path.unwrap().as_str());
    }
    return redis_key;
}
