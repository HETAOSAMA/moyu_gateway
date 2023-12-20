use redis::{Connection, RedisResult};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use tokio::task;

pub struct RedisServer {
    connection: Connection,
}

impl RedisServer {
    pub fn new(redis_url: &str, db: Option<u16>) -> RedisResult<Self> {
        let client = redis::Client::open(redis_url)?;
        let mut connection = client.get_connection()?;
        if let Some(db) = db {
            let _: () = redis::cmd("SELECT")
                .arg(db)
                .query(&mut connection)?;
        }
        Ok(RedisServer { connection })
    }
    // 如果不需要失效时间，可以将 ttl 参数设置为 None。
    // 如果需要设置失效时间，可以将 ttl 参数设置为 Some(Duration::from_secs(60))，表示 60 秒后失效。
    pub async fn set<T>(&self, key: &str, value: &T, ttl: Option<Duration>) -> RedisResult<()>
        where
            T: Serialize,
    {
        let serialized = serde_json::to_string(value)?;
        let connection = self.connection.clone();

        task::spawn(async move {
            let _: () = redis::cmd("SET")
                .arg(key)
                .arg(serialized)
                .query_async(connection)
                .await?;

            if let Some(duration) = ttl {
                let _: () = redis::cmd("EXPIRE")
                    .arg(key)
                    .arg(duration.as_secs() as usize)
                    .query_async(connection)
                    .await?;
            }

            Ok(())
        })
            .await
            .unwrap()
    }

    // 获取需要的结构体时，
    // let retrieved_struct: Option<MyStruct> = redis_util.get("my_key").await.expect("Failed to get object");
    pub async fn get<T>(&self, key: &str) -> RedisResult<Option<T>>
    // 使用 DeserializeOwned 约束的目的是允许对 T 进行反序列化操作，以便从 Redis 中检索数据时能够将存储的字符串转换回对应的 Rust 类型。
        where
            T: DeserializeOwned,
    {
        let connection = self.connection.clone();

        task::spawn(async move {
            let serialized: Option<String> = redis::cmd("GET")
                .arg(key)
                .query_async(connection)
                .await?;

            Ok(serialized.map(|s| serde_json::from_str(&s)).transpose()?)
        })
            .await
            .unwrap()
    }
}
