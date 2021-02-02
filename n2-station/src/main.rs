use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    dev::Service,
    http::header::{HeaderValue, ACCESS_CONTROL_ALLOW_ORIGIN},
    middleware::Logger,
    web, App, HttpServer,
};
use anyhow::Result;
use argh::FromArgs;
use log::LevelFilter;

mod backend;

use backend::RBATIS;

#[derive(FromArgs)]
#[argh(description = "N2Station Backend Startup Parameter")]
struct Param {
    #[argh(
        option,
        description = "config file path",
        default = "String::from(\"config.json\")"
    )]
    config: String,
}

#[derive(serde::Deserialize, Clone)]
struct ServerConfig {
    bind_address: String,
    server_port: u16,
    database_url: String,
    room_creation_limit: u64,
    room_open_limit: u64,
    authorization_force_https: bool,
    debug: bool,
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::builder()
        .default_format()
        .filter_level(LevelFilter::Info)
        .init();

    let param: Param = argh::from_env();

    let config: ServerConfig = web::block(move || {
        let file_path = std::path::Path::new(&param.config);
        let file = std::fs::File::open(file_path).unwrap();
        let reader = std::io::BufReader::new(file);
        serde_json::from_reader(reader)
    })
    .await?;

    RBATIS.link(&config.database_url).await?;
    initialize_database().await?;

    let port = config.server_port;
    let address = config.bind_address.clone();

    HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                let cfg = req.app_data::<web::Data<ServerConfig>>().unwrap().clone();
                let fut = srv.call(req);
                async move {
                    let mut res = fut.await?;
                    if cfg.debug {
                        res.headers_mut()
                            .insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
                    }
                    Ok(res)
                }
            })
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("Authorization")
                    .secure(config.authorization_force_https),
            ))
            .data(config.clone())
            .configure(backend::init)
    })
    .bind((address, port))?
    .run()
    .await?;

    Ok(())
}

async fn initialize_database() -> Result<()> {
    RBATIS
        .exec(
            "",
            r#"
    CREATE TABLE IF NOT EXISTS `users`(
        `uuid` CHAR(32) NOT NULL,
        `username` VARCHAR(16) NOT NULL,
        `email` VARCHAR(30) NOT NULL,
        `passwd` CHAR(60) NOT NULL
    )
    "#,
        )
        .await?;

    RBATIS
        .exec(
            "",
            r#"
    CREATE TABLE IF NOT EXISTS `rooms`(
        `owner_uuid` CHAR(32) NOT NULL,
        `stream_id` VARCHAR(16) NOT NULL,
        `title` VARCHAR(16) NOT NULL,
        `description` VARCHAR(20) NOT NULL,
        `tag` VARCHAR(1024) NULL,
        `open` BOOL NOT NULL,
        `stream_token` CHAR(32) NULL
    )
    "#,
        )
        .await?;

    RBATIS
        .exec(
            "",
            r#"
        UPDATE rooms SET open=FALSE
        "#,
        )
        .await?;

    Ok(())
}
