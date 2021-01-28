use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware::Logger, web, App, HttpServer};
use anyhow::Result;
use argh::FromArgs;
use dotenv::dotenv;
use log::LevelFilter;
use sqlx::{mysql::MySqlPoolOptions, query, MySqlPool};

mod backend;

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
    pool_max_conns: u32,
    server_port: u16,
    database_url: String,
    room_creation_limit: u32,
    room_open_limit: u32,
    authorization_force_https: bool,
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    env_logger::builder()
        .default_format()
        .filter_level(LevelFilter::Info)
        .init();

    let param: Param = argh::from_env();

    let config: ServerConfig = web::block(|| {
        let file = std::fs::File::open(param.config).unwrap();
        let reader = std::io::BufReader::new(file);
        serde_json::from_reader(reader)
    })
    .await?;

    let mysql_pool = MySqlPoolOptions::new()
        .max_connections(config.pool_max_conns)
        .connect(&config.database_url)
        .await?;

    initialize_database(&mysql_pool).await?;

    let port = config.server_port;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("Authorization")
                    .secure(config.authorization_force_https),
            ))
            .data(mysql_pool.clone())
            .configure(backend::init)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await?;

    Ok(())
}

async fn initialize_database(pool: &MySqlPool) -> Result<()> {
    query!(
        r#"
        CREATE TABLE IF NOT EXISTS `users`(
            `uuid` CHAR(32) NOT NULL,
            `username` VARCHAR(16) NOT NULL,
            `email` VARCHAR(30) NOT NULL,
            `passwd` BINARY(60) NOT NULL,
            `reg_date` TIMESTAMP NOT NULL
        )
    "#
    )
    .execute(pool)
    .await?;

    query!(
        r#"
        CREATE TABLE IF NOT EXISTS `rooms`(
            `owner_uuid` CHAR(32) NOT NULL,
            `stream_id` VARCHAR(16) NOT NULL,
            `title` VARCHAR(16) NOT NULL,
            `desc` VARCHAR(20) NOT NULL,
            `tag` VARCHAR(1024) NULL,
            `open` BOOL NOT NULL,
            `stream_token` CHAR(32) NULL
        )
        "#
    )
    .execute(pool)
    .await?;

    query!(
        r#"
        CREATE TABLE IF NOT EXISTS `tags`(
            `id` INT UNSIGNED AUTO_INCREMENT,
            `tag_type` VARCHAR(10) NOT NULL,
            `creator_uuid` CHAR(32) NOT NULL,
            PRIMARY KEY ( id )
        )
        "#
    )
    .execute(pool)
    .await?;

    query!(
        r#"
        UPDATE rooms SET open=FALSE
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}
