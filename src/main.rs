use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::middleware::Logger;
use moyu_gateway::controller::sys_services_controller::sys_service_config;
use moyu_gateway::service::CONTEXT;

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .insert_header(("Cache-Control", "no-cache"))
        .body("Hello !")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //log
    let a = moyu_gateway::config::log::init_log();

    //database
    CONTEXT.init_database().await;
    // table::sync_tables(&CONTEXT.rb).await;
    // table::sync_tables_data(&CONTEXT.rb).await;
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .configure(sys_service_config)
    })
        .bind(&CONTEXT.config.server_url)?
        .run()
        .await
}