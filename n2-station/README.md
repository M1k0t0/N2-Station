# N2-Station rust-base Backend

> Notice that the response format is silightly different from that in Wiki

## How to use

All user interface is under /api/ routes
Nginx interface defined in /callback/nginx

For nginx:

```conf
rtmp {
    ...
    notify_method get;
    application {
        ...
        on_publish http://localhost:8080/callback/nginx;
    }
}
```

Then the user should have their room created, opened and push to `/room?token=` where the token goes after `=`

## Configuration

Information needed to run the Backend is contained in `config.json`, which should be sharing the same folder with the binary by default. You can change the configuration with `--config <path>` argument.

```json5
{
    "bind_address": "0.0.0.0",
    "pool_max_conns": 10, //Max connection to sql pool
    "server_port": 8080, //The port the server runs on
    "database_url":"<url>", //e.g. mariadb://root:qwer1234@127.0.0.1/n2station
    "room_creation_limit": 5, //Limit number of room created per user
    "room_open_limit": 2, //The number of room that could be open status globally
    "authorization_force_https": true, //Wheter authorization requires https forcibly, if true, no cookies will be set
}
```

## Usage of prebuilt

You may have either of `MariaDB` or `MySQL` installed. Then run the following SQL:

> You have to create a database for the Backend to work correctly. You can choose any name you prefer, but it have to match with the configuration.

```sql
# Replace <database> with any name you prefer
CREATE DATABASE <database>;
```

## Build

> Usage of prebuilt version is recommended

Installation of cargo and rustc is required.
See [rust-lang.org](https://www.rust-lang.org/tools/install)

### Linux

#### Requirememnts

- g++
- pkg-config
- libssl-dev

#### Setup Database

You may have either of `MariaDB` or `MySQL` installed. Then run the following SQLs:

```sql
# Replace <database> with any name you prefer
CREATE DATABASE <database>;
# SQL schemas will be checked during compile time
# Thus we need to create tables manually
CREATE TABLE IF NOT EXISTS `users`(
            `uuid` CHAR(32) NOT NULL,
            `username` VARCHAR(16) NOT NULL,
            `email` VARCHAR(30) NOT NULL,
            `passwd` BINARY(60) NOT NULL,
            `reg_date` TIMESTAMP NOT NULL
        );
CREATE TABLE IF NOT EXISTS `rooms`(
            `owner_uuid` CHAR(32) NOT NULL,
            `stream_id` VARCHAR(16) NOT NULL,
            `title` VARCHAR(16) NOT NULL,
            `desc` VARCHAR(20) NOT NULL,
            `tag` VARCHAR(1024) NULL,
            `open` BOOL NOT NULL,
            `stream_token` CHAR(32) NULL
        );
```

#### Setup Enviroment

Edit `.env` file as following
`DATABASE_URL=<database_url>`

Example given in default `.env`
__DON'T FORGET TO EDIT THE CONFIGURATION FILE__

#### Build Command

`cargo build --release`

Product could be found in ./target/release/

### Windows

Simply run `cargo build --release`

## API

Too lazy to write it. Just have a look at `routes.rs` and `enum Action` in `model.rs`

## TODO

- More appropriate error handling and rensponse
- OAuth for authentication
- Support streaming natively
