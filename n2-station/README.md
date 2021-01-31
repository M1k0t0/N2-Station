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
        on_publish http://127.0.0.1:8080/callback/nginx;
    }
}
```

Then the user should have their room created, opened and push to `/room?token=` where the token goes after `=`

## Configuration

Information needed to run the Backend is contained in `config.json`, which should be sharing the same folder with the binary by default. You can change the configuration with `--config <path>` argument.

```json5
{
    "bind_address": "0.0.0.0",
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

#### Build Command

`cargo build --release`

Product could be found in ./target/release/

### Windows

Run `cargo build --release`

## API

Too lazy to write it. Just have a look at `routes.rs` and `enum Action` in `model.rs`

## TODO

- More appropriate error handling and rensponse
- OAuth for authentication
- Support streaming natively
