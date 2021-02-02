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
    "database_url":"<url>", //e.g. mysql://root:qwer1234@127.0.0.1/n2station
    "room_creation_limit": 5, //Limit number of room created per user
    "room_open_limit": 2, //The number of room that could be open status globally
    "authorization_force_https": true, //Wheter authorization requires https forcibly, if true with http, no cookies will be set
    "debug": false //enable debug mode - Access-Control-Allow-Origin: * will be appended to headers
}
```

## Usage of prebuilt

You may have either of `MariaDB` or `MySQL` installed. Then run the following SQL:

```sql
# Replace <database> with any name you prefer
CREATE DATABASE <database>;
```

> You have to create a database for the Backend to work correctly. You can choose any name you prefer, but it have to match with the configuration.

## Build

> Usage of prebuilt version is recommended

Installation of cargo and rustc is required.
See [rust-lang.org](https://www.rust-lang.org/tools/install)

### Linux

#### Requirements

- g++
- pkg-config
- libssl-dev

#### Build Command

`cargo build --release`

Product could be found in ./target/release/

### Windows

Run `cargo build --release`

## API

Frontend-oriented API

### Overview

#### Route

`/api/<category>/<name>`

#### Categories

- [info](#info) - Requests to fetch universally accessible informations
- [user](#user) - User operation and user-only informations
- [auth](#auth) - Authorization-related operations

#### Response Format

All effective requests to `/api/..` return an unformatted json, following the next format

```json5
{
    //data goes here
    //...
    "action": "actionName" //camelCase
}
```

#### Authorization & Security

User's authorization token is stored in cookies under `Authorization`.

__!!!WARNING__ We store bcrypted password in databse, so __RAW__ password is transferred with `POST`. But so far out session has __NEVER__ been actively encrypted. `POST` data is __NOT__ encrypted under `http`, and thus you __MUST__ use `https` to ensure the security. This may be solved after OAuth2 implementation is done.

### References

__NOTE: ONLY `application/x-www-form-urlencoded` IS ACCEPTED FOR `POST` REQUEST__

> API(s) notated with (*) means that the API requires authorization first. Otherwise HTTP403(Forbidden) will be responded

#### Common Data Structure

- Simple User

```json5
{
    "id": "<hyphenated-uuid>", //Unique id of user
    "name": "<string>" //Nickname of user
}
```

- User

```json5
{
    "id": "<hyphenated-uuid>", //Unique id of user
    "name": "<string>", //Nickname of user
    "email": "example@example.com" //E-mail of user
}
```

- Room

> Note that `Room` could be __detailed__ or not __detailed__, which affects the visibility of `stream_token`. This property is notated in the API underneath

```json5
{
    "stream_id": "<string>", //The ID of the room where user should stream
    "title": "<string>", //Room title
    "desc": "<string>", //Room description
    "tag": ["<string>"], //Room tags, may be `null`
    "status": "<open/close>", //Room status
    "user": "<simple-user>", //Owner information
    "stream_token": "<hyphenated-uuid>"//Verification token of the room. `null` if not detailed
}
```

#### INFO

- `GET` `/api/info/room`
  - __EXPLANATION__
  Fetch all existing rooms
  - __REQUEST__
  None
  - __RESPONSE__

```json5
{
    "rooms": ["<room(undetailed)>"],
    "action": "getRoomList"
}
```

- `POST` `/api/info/room`
  - __EXPLANATION__
  Fetch informations for a specified room
  - __REQUEST__
    - id - `stream_id` of the room
  - __RESPONSE__

```json5
{
    "room": "<room(undetailed)>", // `null` if not found
    "action": "searchRoom"
}
```

- `GET` `/api/info/user`
  - __EXPLANATION__
  Fetch all existing users
  - __REQUEST__
  None
  - __RESPONSE__

```json5
{
    "users": ["<user>"],
    "action": "getUserList"
}
```

- `POST` `/api/info/user`
  - __EXPLANATION__
  Fetch informations for a specified user
  - __REQUEST__
  Either of the following is required. If both, `id` takes the priority. If neither, `null` is returned
    - id - user's uuid
    - name - user's name
  - __RESPONSE__

```json5
{
    "user": "<user>", // or null
    "action": "searchUser"
}
```

#### USER __(*)__

- `GET` `/api/user/room`
  - __EXPLANATION__
  Fetch rooms owned by user. Thus authorization required
  - __REQUEST__
  None
  - __RESPONSE__

```json5
{
    "rooms": ["<room(undetailed)>"],
    "action": "getUserRoomList"
}
```

- `POST` `/api/user/room`
  - __EXPLANATION__
  Fetch detailed room information for a specific room. This contains `stream_token` so authorization is required. If no such room owned by the user is found, `null` is returned
  - __REQUEST__
    - id - `stream_id` to search
  - __RESPONSE__

```json5
{
    "room": "<room(detailed)>", //or null
    "action": "userRoomDetail"
}
```

- `POST` `/api/user/createRoom`
  - __EXPLANATION__
  Create a room with given data
  - __REQUEST__
    - id - `stream_id` to create(at least 4 and up to 16 letters, contains only `A-Z`, `a-z`, `0-9`, `-` and `_`)
    - title - room title(up to 16 letters)
    - desc - room description(up to 20 letters)
    - tag - room tags (JSON string array e.g. "[\"tagA\", \"tagB\"]"), up to 20 tags, each tag should contain no `;` and no more than 20 characters, optional
  - __RESPONSE__

```json5
{
    "status": "<int>", // 0 for success, -1 if stream_id already exists, -2 if room creation limit exceeded, -10 if parameter missing or invalid
    "action": "createRoom"
}
```

- `POST` `/api/user/deleteRoom`
  - __EXPLANATION__
  Delete specified room
  - __REQUEST__
    - id - `stream_id` to delete
  - __RESPONSE__

```json5
{
    "status": "<int>", // 0 for success, -1 if user doesn't own such room, -10 if parameter missing
    "action": "deleteRoom"
}
```

- `POST` `/api/user/openRoom`
  - __EXPLANATION__
  Open a room to make it ready to push, and return `stream_token`
  - __REQUEST__
    - id - `stream_id` to open
  - __RESPONSE__

```json5
{
    "status": "<int>", // 0 for success, -1 if user doesn't own such room, -2 if room open limit exceeded, -10 if parameter missing
    "stream_token": "<hyphenated-uuid>",
    "action": "openRoom"
}
```

- `POST` `/api/user/closeRoom`
  - __EXPLANATION__
  Close a room and unset `stream_token`
  - __REQUEST__
    - id - `stream_id` to close
  - __RESPONSE__

```json5
{
    "status": "<int>", // 0 for success, -1 if user doesn't own such room, -10 if parameter missing
    "action": "closeRoom"
}
```

#### AUTH

- `POST` `/api/auth/register`
  - __EXPLANATION__
  Register a new account, will __NOT__ set `Authorization` cookies
  - __REQUEST__
    - user - username to create(at least 4 and up to 16 letters, contains only `A-Z`, `a-z`, `0-9`, `-` and `_`)
    - email - email binded to the account
    - pass - password(at least 8 and up to 16 characters, must contain at least one alphabet, one number and one special character)
  - __RESPONSE__

```json5
{
    "status": "<int>", // 0 for success, -1 if username or email occupied, -10 if parameter missing or invalid
    "action": "register"
}
```

- `POST` `/api/auth/getToken`
  - __EXPLANATION__
  Login, will set `Authorization` cookies
  - __REQUEST__
    - email - email binded to the account
    - key - password(at least 8 and up to 16 characters, must contain at least one alphabet, one number and one special character)
  - __RESPONSE__

```json5
{
    "status": "<int>", // 0 for success, -1 if email and password mismatched, -10 if parameter missing or invalid
    "action": "getToken"
}
```

- `GET` `/api/auth/destroyToken`
  - __EXPLANATION__
  Logout, will unset `Authorization` cookies
  - __REQUEST__
  None
  - __RESPONSE__

```json5
{
    "status": "<int>", // 0 for success
    "action": "destroyToken"
}
```

- `POST` `/api/auth/changePassword` __(*)__
  - __EXPLANATION__
  Change password for current user
  - __REQUEST__
    - old_pass - old password(at least 8 and up to 16 characters, must contain at least one alphabet, one number and one special character)
    - new_pass - new password(...)
  - __RESPONSE__

```json5
{
    "status": "<int>", // 0 for success, -1 if old password incorrect, -2 if server encountered internal error, -10 if parameter missing or invalid
    "action": "register"
}
```

## TODO

- More appropriate error handling and rensponse
- OAuth for authentication
- Support streaming natively
- Danmaku system implementation
