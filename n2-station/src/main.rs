use std::env;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{App, HttpServer};
use anyhow::Result;
use argh::FromArgs;
use dotenv::dotenv;
use sqlx::{mysql::MySqlPoolOptions, query, MySqlPool};

mod backend;

#[derive(FromArgs)]
#[argh(description = "N2Station Backend Startup Parameter")]
struct Param {
    #[argh(option, description = "database server port", default = "8848")]
    server_port: u16,
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let param: Param = argh::from_env();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let mysql_pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await?;

    initialize_database(&mysql_pool).await?;

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("Authorization")
                    .secure(false), //true
            ))
            .data(mysql_pool.clone())
            .configure(backend::init)
    })
    .bind(("127.0.0.1", param.server_port))?
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
