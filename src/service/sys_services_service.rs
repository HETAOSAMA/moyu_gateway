use rbatis::{html_sql, htmlsql_select_page, Page, PageRequest, RBatis};
use rbatis::rbdc::DateTime;
use rbatis::rbdc::db::ExecResult;
use rbatis::snowflake::new_snowflake_id;

use serde_json::from_str;
use url::{Url};
use crate::domain::table::sys_services::SysServices;
use crate::domain::vo::reqvo::sys_service_reqvo::{SelectServiceByPageReqVO, UpdateIsActive};
use crate::pool;
use crate::error::Result;
use crate::service::redis_service::{del, get, set};
use crate::error::Error;

pub struct SysServiceService {}

impl SysServiceService {
    #[html_sql("src/domain/table/sys_services.html")]
    pub async fn delete_by_ids(rb: &RBatis,arg: Vec<String>) -> ExecResult {
        impled!()
    }
    #[html_sql("src/domain/table/sys_services.html")]
    pub async fn select_by_ids(rb: &RBatis,arg: Vec<String>) -> Vec<SysServices> {
        impled!()
    }
    htmlsql_select_page!(select_by_page(server_name: &str, is_active: &i32) -> SysServices => "src/domain/table/sys_services.html");

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
                arg.url = Option::from(parsed_url.unwrap().host_str().unwrap().to_string());
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
                if key.clone().unwrap().is_empty() {
                    return Err(Error::from("redis key 生成失败"));
                }
                //查看redis中是否存在
                let value = get(key.clone().unwrap().to_string()).await.await;
                if value.unwrap().is_empty(){
                    if arg.is_active == Some(1) {
                        let _ = set(key.clone().unwrap().to_string(), json_string).await.await;
                    }
                    Ok(SysServices::insert(pool!(), &arg).await?.rows_affected)
                }else { 
                    return Err(Error::from("已有同名服务名称和路径,请重新命名"));
                }
            }
            Err(e) => {
                return Err(Error::from(e.to_string()));
            }
        }
    }

    pub async fn update_service(&self, mut arg: SysServices) -> Result<u64> {
        let current_datetime = DateTime::now();
        arg.updated_at = Some(current_datetime);
        let mut data = SysServices::select_by_id(pool!(), &arg.id.clone().unwrap()).await?.unwrap();
        let redis_key_by_select = get_redis_key(data.clone());
        let redis_key_by_arg = get_redis_key(arg.clone());
        let json_str = get(redis_key_by_select.clone().unwrap().to_string()).await.await;
        let sys_service:SysServices = from_str(json_str.unwrap().as_str()).unwrap();

        //判断服务名称和url是否修改，如果key一样，判断redis取出来的id是否一样，如果一样就修改，不一样就不修改
        if redis_key_by_select.clone().unwrap() == redis_key_by_arg.clone().unwrap() {
            if sys_service.id.unwrap() == arg.clone().id.unwrap() {
                let flag = SysServices::update_by_column(pool!(), &arg, "id").await?.rows_affected;
                let data = SysServices::select_by_id(pool!(), &arg.id.clone().unwrap()).await?.unwrap();
                let _ = set(redis_key_by_arg.unwrap().to_string(), serde_json::to_string(&data).unwrap()).await.await;
                return Ok(flag);
            }else {
                return Err(Error::from("已有同名服务名称和路径,请重新命名"));
            }
        }else {
            let json_str = get(redis_key_by_arg.clone().unwrap()).await.await;
            if json_str.clone().unwrap().is_empty() {
                let flag = SysServices::update_by_column(pool!(), &arg, "id").await?.rows_affected;
                let data = SysServices::select_by_id(pool!(), &arg.id.clone().unwrap()).await?.unwrap();
                let _ = set(redis_key_by_arg.unwrap().to_string(), serde_json::to_string(&data).unwrap()).await.await;
                let _ = del(redis_key_by_select.clone().unwrap().to_string()).await.await;
                return Ok(flag);
            }else {
                return Err(Error::from("已有同名服务名称和路径,请重新命名"));
            }
        }
    }

    pub async fn delete_service_by_ids(&self, arg: Vec<String>) -> Result<u64> {
        let datas = SysServiceService::select_by_ids(pool!(), arg.clone()).await?;
        if !datas.is_empty(){
            for data in datas {
                let redis_key = get_redis_key(data);
                del(redis_key.unwrap()).await.await.unwrap();
            }
        }
        let a = SysServiceService::delete_by_ids(pool!(), arg).await?.rows_affected;
        Ok(a)
    }
    pub async fn select_service_by_page(&self, arg: &SelectServiceByPageReqVO) -> Result<Page<SysServices>> {
        let a = SysServiceService::select_by_page(pool!(),
                               &PageRequest::new(arg.page_no.clone().unwrap_or_default(), arg.page_size.clone().unwrap_or_default()),
                               &arg.server_name.as_deref().unwrap_or_default(),
                               &arg.is_active.unwrap_or_default()).await?;
        let page = Page::<SysServices>::from(a);
        return Ok(page);
    }

    pub async fn update_is_active(&self,arg: &UpdateIsActive) -> Result<u64> {
        let current_datetime = DateTime::now();
        let mut data = SysServices::select_by_id(pool!(), &arg.id.clone().unwrap()).await?.unwrap();
        if arg.is_active !=Some(1) {
            let redis_key = get_redis_key(data.clone());
            del(redis_key.unwrap()).await.await.unwrap();
            data.is_active = arg.is_active.clone();
            data.updated_at = Some(current_datetime);
            let flag = SysServices::update_by_column(pool!(), &data, "id").await?.rows_affected;
            return Ok(flag);
        }else {
            let redis_key = get_redis_key(data.clone());
            data.is_active = arg.is_active.clone();
            data.updated_at = Some(current_datetime);
            match set(redis_key.unwrap().to_string(), serde_json::to_string(&data).unwrap()).await.await{
                Ok(_) => {
                    let flag = SysServices::update_by_column(pool!(), &data, "id").await?.rows_affected;
                    return Ok(flag);
                }
                Err(e) => {
                    return Err(Error::from(e.to_string()));
                }
            }
        }
    }
}


//生成redis key
pub fn get_redis_key(mut arg: SysServices) -> Option<String> {
    let mut redis_key = String::new();
    if arg.server_name.is_some() {
        if arg.path.is_some() {
            redis_key = format!("{}{}", arg.server_name.unwrap(), arg.path.unwrap());
        } else{
            redis_key = format!("{}", arg.server_name.unwrap());
        }
    }
    return Some(redis_key);
}
