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
    pub port: Option<u16>,
    // 服务的路径，用于指定服务的特定子路径。可选<i32>,
    pub path: Option<String>,
    // 服务是否激活，默认为1 （1-启用，2-禁用）
    pub is_active: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateServiceReqVO {
    // 必填
    pub id: Option<String>,
    // 必填
    pub server_name: Option<String>,
    // 必填
    pub url: Option<String>,
    pub description: Option<String>,
    pub protocol: Option<String>,
    pub port: Option<u16>,
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteServiceReqVO {
    pub ids: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SelectServiceByPageReqVO {
    pub server_name: Option<String>,
    // 必填服务是否激活，1-启用，2-禁用，0-全部
    pub is_active: Option<i32>,
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateIsActive {
    // 必填
    pub id: Option<String>,
    // 必填服务是否激活，1-启用，2-禁用
    pub is_active: Option<i32>,
}
