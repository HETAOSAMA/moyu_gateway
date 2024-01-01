pub mod redis_service;
mod sys_services_service;

use once_cell::sync::Lazy;
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;
use redis::{Client, ConnectionLike};
use crate::config::config::ApplicationConfig;
use crate::service::redis_service::RedisService;
use crate::service::sys_services_service::SysServiceService;

pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());

#[macro_export]
macro_rules! pool {
    () => {
        &$crate::service::CONTEXT.rb
    };
}
#[macro_export]
macro_rules! get_rd {
    () => {
        &mut $crate::service::CONTEXT.redis_service.get_conn().await.unwrap()
    };
}


pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rb: RBatis,
    pub sys_service_service: SysServiceService,
    pub redis_service: RedisService,
}

impl ServiceContext {
    pub async fn init_database(&self) {
        log::info!(
            "rbatis pool init ({})...",
            self.config.database_url
        );
        self.rb
            .link(MysqlDriver{},&self.config.database_url).await
            .expect("rbatis link error");
        log::info!(
            "rbatis pool init success! pool state = {}",
            self.rb.get_pool().expect("pool not init!").state().await
        );
        log::info!(
            " - Local: http:/{}",
            self.config.server_url
        );
    }
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        ServiceContext {
            rb: RBatis::new(),
            config: config.clone(),
            sys_service_service: SysServiceService{},
            redis_service: RedisService::new(config.redis_url)
        }
    }
}