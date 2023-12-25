use lazy_static::lazy_static;
use log::{error, info};
use redis::{Client, Commands, Connection};
use crate::config::config::ApplicationConfig;

lazy_static! {
    static ref REDIS_CLIENT: Client = init_redis_client();
}

fn get_redis_client<'a>() -> &'a Client {
    &REDIS_CLIENT
}

fn init_redis_client() -> Client {
    let redis_url = ApplicationConfig::default().redis_url;
    let cluster = &redis_url;
    info!("redis cluster: {:?}", cluster);
    Client::open(cluster.to_string()).unwrap()
}

fn get_redis_conn() -> Connection {
    let client = get_redis_client();
    let connection = client.get_connection().unwrap();
    connection
}

pub fn set(key: String, value: String) -> bool {
    get_redis_conn()
        .set::<String, String, bool>(key, value)
        .unwrap_or_else(|err| {
            error!("Failed to set value in Redis: {:?}", err);
            false
        })
}

pub fn set_nx(key: String, value: String) -> bool {
    get_redis_conn()
        .set_nx::<String, String, bool>(key, value)
        .unwrap_or_else(|err| {
            error!("Failed to set_nx value in Redis: {:?}", err);
            false
        })
}

pub fn set_ex(key: String, value: String, seconds: i32) -> bool {
    get_redis_conn()
        .set_ex::<String, String, bool>(key, value, seconds as usize as u64)
        .unwrap_or_else(|err| {
            error!("Failed to set_ex value in Redis: {:?}", err);
            false
        })
}

pub fn get(key: String) -> String {
    get_redis_conn()
        .get::<String, String>(key)
        .unwrap_or_else(|err| {
            error!("Failed to get value from Redis: {:?}", err);
            "".to_string()
        })
}

pub fn del(key: String) -> bool {
    get_redis_conn()
        .del::<String, bool>(key)
        .unwrap_or_else(|err| {
            error!("Failed to del value from Redis: {:?}", err);
            false
        })
}