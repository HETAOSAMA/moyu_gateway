use actix_web::{HttpRequest, HttpResponse, web};
use futures_util::StreamExt;
use serde_json::from_str;
use url::{ParseError, Url};
use reqwest::Client;
use reqwest::header::HeaderMap;
use std::time::Duration;
use crate::domain::table::sys_services::SysServices;
use crate::service::redis_service::get;

async fn handle_specific_path(service_info: String, path: &str,req: HttpRequest,body: web::Bytes) -> Result<HttpResponse, actix_web::Error>{
    // 解析服务信息
    let sys_services:SysServices = from_str(&service_info).unwrap();
    let domain = sys_services.url.unwrap();
    let port: Option<u16> = sys_services.port;
    let protocol = sys_services.protocol.unwrap();
    let mut target_url = String::new();
    // 构建目标URL
    if port.is_none() {
        target_url = format!("{}://{}/{}", protocol, domain, path);
    }else {
        target_url = format!("{}://{}:{}/{}", protocol, domain, port.unwrap(), path);
    }
    // 获取查询参数
    let query_params: Vec<_> = req.query_string().split('&').collect();

    // 添加查询参数到目标 URL
    if !query_params.is_empty() && !query_params[0].is_empty(){
        target_url.push_str("?");
        target_url.push_str(&query_params.join("&"));
    }

    println!("target_url:{:?}",target_url);

    // 设置跨域头
    let response = HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "*"))
        .append_header(("Access-Control-Allow-Methods", "GET, POST, OPTIONS"))
        .append_header(("Access-Control-Allow-Headers", "Content-Type"))
        .append_header(("Access-Control-Max-Age", "3600"))
        .body("OK");

    // 处理OPTIONS请求，直接返回跨域头
    if req.method() == "OPTIONS" {
        return Ok(response);
    }
    // 获取原始请求的头信息
    let original_headers = req.headers().clone();
    // 设置超时时间为5秒
    let timeout = Duration::from_secs(5000);

    // 构建转发请求
    let client = match Client::builder().timeout(timeout).build(){
        Ok(client) => client,
        Err(err) => return Err(actix_web::error::ErrorBadRequest(err)),
    };
    println!("body:{:?}",body);
    let mut target_request = client
        .request(req.method().clone(), target_url)
        .headers(HeaderMap::from(original_headers));
    if !body.is_empty() {
        target_request = target_request.body(body);
    }
    println!("target_request:{:?}",target_request);
    // 发送转发请求
    let target_response = match target_request.send().await{
        Ok(response) => response,
        Err(err) => return Err(actix_web::error::ErrorBadRequest(err)),
    };
    println!(
        "target_response:{:?}",
        target_response
    );
    // 获取目标服务的响应头
    let target_headers = target_response.headers().clone();
    // 构建响应，将目标服务的响应返回给客户端
    let mut response = HttpResponse::build(target_response.status());
    for (key, value) in target_headers.iter() {
        response.append_header((key.clone(), value.clone()));
    }
    let response_body = match target_response.bytes().await{
        Ok(bytes) => bytes,
        Err(err) => return Err(actix_web::error::ErrorBadRequest(err)),
    };
    Ok(response.body(response_body))
}

// 主处理函数
pub async fn handle_request(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    body: web::Bytes,
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
            return handle_specific_path(service_info.clone(), &path,req,body).await;
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
                // println!("wildcard_key:{}",wildcard_key);
                let wildcard_result: Result<String, String> = get(wildcard_key.to_string()).await.await.map_err(|_| "Service not found".to_string());
                if let Ok(ref service_info) = wildcard_result {
                    // 处理通配符路径
                    if !wildcard_result.clone().unwrap().is_empty() {
                        return handle_specific_path(service_info.clone(), &path,req,body).await;
                    }
                }
            }
        }
    }
    return Ok(HttpResponse::NotFound().body("Service not found"));
}
