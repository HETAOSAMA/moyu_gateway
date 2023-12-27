use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AddServiceReqVO {
    // 服务的名称，用于标识服务。
    pub server_name: Option<String>,
    // 服务的 URL 地址，存储服务的访问地址。
    pub url: Option<String>,
    // 对服务的描述，可选字段，用于提供更多信息。
    pub description: Option<String>,
    // 服务的协议，例如 HTTP、HTTPS。
    pub protocol: Option<String>,
    // 服务的端口号可选
    pub port: Option<i32>,
    // 服务的路径，用于指定服务的特定子路径。可选<i32>,
    pub path: Option<String>,
    // 服务是否激活，默认为1 （1-启用，2-禁用）
    pub is_active: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateServiceReqVO {
    pub id: Option<String>,
    pub server_name: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub protocol: Option<String>,
    pub port: Option<i32>,
    pub path: Option<String>,
    pub is_active: Option<i32>,
}

