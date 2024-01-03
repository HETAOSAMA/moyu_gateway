use actix_web::{HttpRequest, HttpResponse, web};
use futures_util::StreamExt;
use serde_json::from_str;
use url::{ParseError, Url};
use crate::domain::table::sys_services::SysServices;
use crate::service::redis_service::get;

// 处理具体路径的逻辑
//思路，先获取服务名称，然后先从redis中查找有无服务名称/*的key,没有就查询所有服务名称的key，有就继续循环去匹配，匹配到就返回，匹配不到就返回404
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
    //获取完整url
    let url = format!("https://www.test.com/{}/{}",service, path);
    let url = match Url::parse(&url) {
        Ok(url) => url,
        Err(ParseError::RelativeUrlWithoutBase) => return Ok(HttpResponse::NotFound().body("Service not found")),
        Err(err) => return Err(actix_web::error::ErrorBadRequest(err)),
    };
// 使用 let 绑定来创建更长寿命的值
    let url_segments = url.path_segments().map(|c| c.collect::<Vec<_>>());

// 判断是否成功获取了 path_segments
    let url_list = match url_segments.clone() {
        Some(segments) => segments,
        None => {
            // 处理url为None的情况
            // 可以返回默认值、提前返回错误等
            return Ok(HttpResponse::NotFound().body("Service not found"));
        }
    };

    // 先匹配服务名称/*
    let wildcard_key = format!("{}/{}", service, "*");
    let wildcard_result: Result<String, String> = get(wildcard_key.to_string()).await.await.map_err(|_| "Service not found".to_string());
    // 如果成功匹配通配符路径
    if let Ok(ref service_info) = wildcard_result {
        // 处理通配符路径
        if !wildcard_result.clone().unwrap().is_empty() {
            return handle_specific_path(service_info.clone(), &path).await;
        }else {
            //循环url_list拼接* 成为key去redis中查询，如果循环到最后一个都没有匹配到那就使用url.path去查询，如果还是没有就返回404
            let mut key = String::new();
            for (index, item) in url_list.iter().enumerate() {
                if index == 0 {
                    let str = url_list.get(index).unwrap();
                    key = str.to_string();
                }else {
                    key = format!("{}/{}", key, url_list.get(index).unwrap().to_string());
                }
                let mut wildcard_key = format!("{}/{}", key, "*");
                if index == url_list.len() - 1 {
                    wildcard_key = key.clone();
                }
                println!("wildcard_key:{}",wildcard_key);
                let wildcard_result: Result<String, String> = get(wildcard_key.to_string()).await.await.map_err(|_| "Service not found".to_string());
                if let Ok(ref service_info) = wildcard_result {
                    // 处理通配符路径
                    if !wildcard_result.clone().unwrap().is_empty() {
                        return handle_specific_path(service_info.clone(), &path).await;
                    }
                }
            }
        }
    }
    return Ok(HttpResponse::NotFound().body("Service not found"));
}
