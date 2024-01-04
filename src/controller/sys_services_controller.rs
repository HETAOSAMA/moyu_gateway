use actix_web::{post, Responder, web};
use crate::domain::table::sys_services::SysServices;
use crate::domain::vo::reqvo::sys_service_reqvo::{AddServiceReqVO, DeleteServiceReqVO, SelectServiceByPageReqVO, UpdateIsActive, UpdateServiceReqVO};
use crate::domain::vo::respvo::RespVO;
use crate::service::CONTEXT;

pub fn sys_service_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/sys_service")
            .service(add_service)
            .service(update_service)
            .service(delete_service_by_ids)
            .service(select_service_by_page)
            .service(update_is_active)
    );
}

#[post("/add_service")]
async fn add_service(arg: web::Json<AddServiceReqVO>) -> impl Responder{
    let sys_service = SysServices{
        id: None,
        server_name: arg.server_name.clone(),
        url: arg.url.clone(),
        description: arg.description.clone(),
        protocol: arg.protocol.clone(),
        port: arg.port.clone(),
        path: arg.path.clone(),
        is_active: arg.is_active.clone(),
        created_at: None,
        updated_at: None,
    };
    let vo = CONTEXT.sys_service_service.add_service(sys_service).await;
    return RespVO::from_result(&vo).resp_json()
}

#[post("/update_service")]
async fn update_service(arg: web::Json<UpdateServiceReqVO>) -> impl Responder{
    let sys_service = SysServices{
        id: arg.id.clone(),
        server_name: arg.server_name.clone(),
        url: arg.url.clone(),
        description: arg.description.clone(),
        protocol: arg.protocol.clone(),
        port: arg.port.clone(),
        path: arg.path.clone(),
        is_active: None,
        created_at: None,
        updated_at: None,
    };
    let vo = CONTEXT.sys_service_service.update_service(sys_service).await;
    return RespVO::from_result(&vo).resp_json()
}
#[post("/update_is_active")]
async fn update_is_active(arg: web::Json<UpdateIsActive>) -> impl Responder{
    let vo = CONTEXT.sys_service_service.update_is_active(&arg).await;
    return RespVO::from_result(&vo).resp_json()
}

#[post("/delete_service_by_ids")]
async fn delete_service_by_ids(arg: web::Json<DeleteServiceReqVO>) -> impl Responder{
    let vo = CONTEXT.sys_service_service.delete_service_by_ids(arg.ids.clone().unwrap()).await;
    return RespVO::from_result(&vo).resp_json()
}

#[post("/select_service_by_page")]
async fn select_service_by_page(arg: web::Json<SelectServiceByPageReqVO>) -> impl Responder{
    let vo = CONTEXT.sys_service_service.select_service_by_page(&arg.0).await;
    return RespVO::from_result(&vo).resp_json()
}

