use actix_web::{HttpServer, App};
use std::process;
use actix_web::middleware::Logger;
use random_team::router::router_config;
use random_team::setup_logger;
use random_team::team::Team;
use actix::Actor;
use actix_web::web::Data;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");

    if let Err(err) = setup_logger() {
        log::error!("日志初始化错误！ {:?}", err);
        process::exit(1);
    }

    let team = Team::new()?.start();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(team.clone()))
            .configure(router_config)
    })
        .bind("0.0.0.0:9699")?
        .run()
        .await
}
