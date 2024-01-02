use actix_web::{HttpRequest, HttpResponse, Responder, web};
use serde_json::from_str;
use crate::domain::table::sys_services;
use crate::domain::table::sys_services::SysServices;
use crate::service::redis_service::get;

// 处理具体路径的逻辑
async fn handle_specific_path(service_info: String, path: &str) -> Result<HttpResponse, actix_web::Error>{
    // 解析服务信息
    println!("service_info:{}",service_info);
    let sys_services:SysServices = from_str(&service_info).unwrap();
    let domain = sys_services.url.unwrap();
    let port: u16 = sys_services.port.unwrap().try_into().unwrap();
    let protocol = sys_services.protocol.unwrap();

    // 构建目标URL
    let target_url = format!("{}://{}:{}/{}", protocol, domain, port, path);

    // 在此可以使用req对象进行进一步的处理，例如将请求转发到目标URL
    // 这里简单返回目标URL作为示例
    Ok(HttpResponse::Ok().body(target_url))
}

// 主处理函数
pub async fn handle_request(
    req: HttpRequest,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, actix_web::Error> {
    // 从路径中提取服务和路径信息
    let (service, path) = path.into_inner();
    // 尝试匹配具体路径
    let specific_key = format!("{}/{}", service, path);
    let json_string = get(specific_key.to_string()).await.await;

    match json_string {
        Ok(ref service_info) => {
            if json_string.clone().unwrap().is_empty() {
                let wildcard_key = format!("{}/{}", service, "*");
                let wildcard_result: Result<String, String> = get(wildcard_key.to_string()).await.await.map_err(|_| "Service not found".to_string());
                // 如果成功匹配通配符路径
                if let Ok(service_info) = wildcard_result {
                    // 处理通配符路径
                    if wildcard_result.unwrap().is_empty() {
                        return Ok(HttpResponse::NotFound().body("Service not found"));
                    }else {
                        return handle_specific_path(service_info, &path).await;
                    }
                } else {
                    // 如果无法匹配具体路径和通配符路径，则返回HTTP 404 Not Found响应
                    Ok(HttpResponse::NotFound().body("Service not found"))
                }
            }else {
                // 处理具体路径
                handle_specific_path(service_info.clone(), &path).await
            }
        }
        Err(_) => {
            return Ok(HttpResponse::NotFound().body("Service not found"));
        }
    }
}
