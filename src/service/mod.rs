mod redis_service;
mod sys_services_service;

use once_cell::sync::Lazy;
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;
use crate::config::config::ApplicationConfig;
use crate::service::sys_services_service::SysServiceService;

pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());

#[macro_export]
macro_rules! pool {
    () => {
        &$crate::service::CONTEXT.rb
    };
}

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rb: RBatis,
    pub sys_service_service: SysServiceService,
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
            config,
            sys_service_service: SysServiceService{},
        }
    }
}