use futures_util::future::BoxFuture;
use log::{error, info};
use crate::error::{Error, Result};
use redis::aio::Connection;
use crate::get_rd;
use std::string::String;
use redis::{AsyncCommands, Client, RedisResult};

// 初始化 Redis 客户端的 Lazy 静态变量
pub struct RedisService {
    pub client: Client,
}
impl RedisService {
    pub fn new(url: String) -> Self {
        info!(
            "redis init ({})...",
            url
        );
        let client = Client::open(url).unwrap();
        info!(
            "redis pool init success!"
        );
        RedisService { client }
    }

    pub async fn get_conn(&self) -> Result<Connection> {
        let conn = self.client.get_async_connection().await;
        if conn.is_err() {
            let err = format!("RedisService connect fail:{}", conn.err().unwrap());
            error!("{}", err);
            return Err(crate::error::Error::from(err));
        }

        Ok(conn.unwrap())
    }
}



pub async fn set(key: String, value: String) -> BoxFuture<'static,Result<bool>> {
    Box::pin(async move {
        let mut connection = get_rd!();
        let result = connection.set(key, value);
        result.await.map_err(|err| {
            error!("Failed to set value in Redis: {:?}", err);
            Error::from(err.to_string())
        })
    })
}

pub async fn set_nx(key: String, value: String) -> BoxFuture<'static,Result<bool>> {
    Box::pin(async move {
        let mut connection = get_rd!();
        let result = connection.set_nx(key, value);
        result.await.map_err(|err| {
            error!("Failed to set_nx value in Redis: {:?}", err);
            Error::from(err.to_string())
        })
    })

}

pub async fn set_ex(key: String, value: String, seconds: u64) -> BoxFuture<'static,Result<bool>> {
    Box::pin(async move {
        let mut connection = get_rd!();
        let result = connection.set_ex(key, value,seconds);
        result.await.map_err(|err| {
            error!("Failed to set_ex value in Redis: {:?}", err);
            Error::from(err.to_string())
        })
    })
}

pub async fn get(key: String) -> BoxFuture<'static,Result<String>> {
    Box::pin(async move {
        let result: RedisResult<Option<String>> =
            redis::cmd("GET").arg(&[&key]).query_async(get_rd!()).await;
        return match result {
            Ok(v) => Ok(v.unwrap_or_default()),
            Err(e) => Err(Error::from(format!(
                "RedisService get_string({}) fail:{}",
                key,
                e.to_string()
            ))),
        };



    })
}

pub async fn del(key: String) -> BoxFuture<'static,Result<usize>> {
    Box::pin(async move {
        let mut connection = get_rd!();
        let result = connection.del(key);
        result.await.map_err(|err| {
            error!("Failed to del value from Redis: {:?}", err);
            Error::from(err.to_string())
        })
    })
}
