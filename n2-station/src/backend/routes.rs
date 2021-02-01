//!TODO: more appropriate error handling

use crate::ServerConfig;

use super::{form, handler, Action};
use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse};
use fancy_regex::Regex;
use form::{ChangePassword, LoginInfo, RegInfo};
use lazy_static::lazy_static;
use rbatis::rbatis::Rbatis;
use uuid::Uuid;
lazy_static! {
    static ref ID_REGEX: Regex = Regex::new("^[A-Za-z0-9_-]{4,16}$").unwrap();
    static ref EMAIL_REGEX: Regex = Regex::new("^[a-zA-Z0-9_-]+@[a-zA-Z0-9_-]+(\\.[a-zA-Z0-9_-]+)+$").unwrap();
    static ref PASSWD_REGEX: Regex = Regex::new("^(?![0-9]+$)(?![a-zA-Z]+$)(?![0-9a-zA-Z]+$)(?![0-9\\W]+$)(?![a-zA-Z\\W]+$)[0-9A-Za-z\\W]{8,16}$").unwrap();
    static ref TITLE_REGEX: Regex = Regex::new("^.{1,16}$").unwrap();
    static ref DESC_REGEX: Regex = Regex::new("^.{1,20}$").unwrap();

    pub static ref RBATIS: Rbatis = Rbatis::new();
}

#[get("/api/info/room")]
async fn get_room_list() -> HttpResponse {
    let result = handler::get_all_rooms().await;
    if let Ok(rooms) = result {
        HttpResponse::Ok().json(Action::GetRoomList { rooms })
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[post("/api/info/room")]
async fn get_room_info(room: web::Form<form::RoomInfo>) -> HttpResponse {
    let result = handler::search_room_by_stream_name(&room.id, "", false).await;
    if let Ok(room) = result {
        HttpResponse::Ok().json(Action::SearchRoom { room })
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[get("/api/info/user")]
async fn get_user_list() -> HttpResponse {
    let result = handler::get_all_users().await;
    if let Ok(users) = result {
        HttpResponse::Ok().json(Action::GetUserList { users })
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[post("/api/info/user")]
async fn get_user_detail(user: web::Form<form::UserInfo>) -> HttpResponse {
    let result = if !user.id.is_empty() {
        if let Ok(uuid) = Uuid::parse_str(user.id.as_str()) {
            handler::search_user_by_uuid(uuid).await
        } else {
            return HttpResponse::BadRequest().finish();
        }
    } else if !user.name.is_empty() {
        handler::search_user_by_name(&user.name).await
    } else {
        Ok(None)
    };
    if let Ok(user) = result {
        HttpResponse::Ok().json(Action::SearchUser { user })
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[get("/api/user/room")]
async fn get_user_rooms(id: Identity) -> HttpResponse {
    if let Some(uuid) = id.identity() {
        let result = handler::search_rooms_by_owner(&uuid).await;
        if let Ok(rooms) = result {
            HttpResponse::Ok().json(Action::GetUserRoomList { rooms })
        } else {
            HttpResponse::InternalServerError().finish()
        }
    } else {
        HttpResponse::Forbidden().finish()
    }
}

#[post("/api/user/room")]
async fn user_room_detail(id: Identity, room: web::Form<form::RoomId>) -> HttpResponse {
    if let Some(uuid) = id.identity() {
        let result = handler::search_room_by_stream_name(&room.id, &uuid, true).await;
        if let Ok(room) = result {
            HttpResponse::Ok().json(Action::UserRoomDetail { room })
        } else {
            HttpResponse::InternalServerError().finish()
        }
    } else {
        HttpResponse::Forbidden().finish()
    }
}

#[post("/api/user/createRoom")]
async fn create_room(
    config: web::Data<ServerConfig>,
    id: Identity,
    room: web::Form<form::RoomCreation>,
) -> HttpResponse {
    if let Some(uuid) = id.identity() {
        if handler::raw_rooms_for_user(&uuid).await.unwrap() > config.room_creation_limit {
            HttpResponse::Ok().json(Action::CreateRoom { status: -2 })
        } else if !ID_REGEX.is_match(&room.id).unwrap()
            || !TITLE_REGEX.is_match(&room.title).unwrap()
            || !DESC_REGEX.is_match(&room.desc).unwrap()
            || room.tag.len() > 10
            || room.tag.iter().any(|s| s.contains(';'))
        {
            HttpResponse::Ok().json(Action::CreateRoom { status: -10 })
        } else {
            if handler::exists_room(&room.id).await {
                HttpResponse::Ok().json(Action::CreateRoom { status: -1 })
            } else {
                if let Ok(_) =
                    handler::create_room(&uuid, &room.id, &room.title, &room.desc, room.tag.clone())
                        .await
                {
                    HttpResponse::Ok().json(Action::CreateRoom { status: 0 })
                } else {
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    } else {
        HttpResponse::Forbidden().finish()
    }
}

#[post("/api/user/deleteRoom")]
async fn delete_room(id: Identity, room: web::Form<form::RoomId>) -> HttpResponse {
    if let Some(uuid) = id.identity() {
        if room.id.is_empty() {
            HttpResponse::Ok().json(Action::DeleteRoom { status: -10 })
        } else {
            if let Ok(affected) = handler::delete_room(&room.id, &uuid).await {
                if affected > 0 {
                    HttpResponse::Ok().json(Action::DeleteRoom { status: 0 })
                } else {
                    HttpResponse::Ok().json(Action::DeleteRoom { status: -1 })
                }
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
    } else {
        HttpResponse::Forbidden().finish()
    }
}

#[post("/api/user/openRoom")]
async fn open_room(
    config: web::Data<ServerConfig>,
    id: Identity,
    room: web::Form<form::RoomId>,
) -> HttpResponse {
    if let Some(uuid) = id.identity() {
        if handler::raw_open_rooms().await.unwrap() > config.room_open_limit {
            HttpResponse::Ok().json(Action::OpenRoom {
                stream_token: Uuid::nil(),
                status: -2,
            })
        } else if room.id.is_empty() {
            HttpResponse::Ok().json(Action::OpenRoom {
                stream_token: Uuid::nil(),
                status: -10,
            })
        } else {
            if let Ok(affected) = handler::set_room_status(&room.id, true, &uuid).await {
                if affected > 0 {
                    let uuid = handler::assign_stream_token(&room.id).await.unwrap();
                    HttpResponse::Ok().json(Action::OpenRoom {
                        stream_token: uuid,
                        status: 0,
                    })
                } else {
                    HttpResponse::Ok().json(Action::OpenRoom {
                        stream_token: Uuid::nil(),
                        status: -1,
                    })
                }
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
    } else {
        HttpResponse::Forbidden().finish()
    }
}

#[post("/api/user/closeRoom")]
async fn close_room(id: Identity, room: web::Form<form::RoomId>) -> HttpResponse {
    if let Some(uuid) = id.identity() {
        if room.id.is_empty() {
            HttpResponse::Ok().json(Action::CloseRoom { status: -10 })
        } else {
            if let Ok(affected) = handler::set_room_status(&room.id, false, &uuid).await {
                if affected > 0 {
                    handler::unset_stream_token(&room.id).await.unwrap();
                    HttpResponse::Ok().json(Action::CloseRoom { status: 0 })
                } else {
                    HttpResponse::Ok().json(Action::CloseRoom { status: -1 })
                }
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
    } else {
        HttpResponse::Forbidden().finish()
    }
}

#[post("/api/auth/register")]
async fn register(user: web::Form<RegInfo>) -> HttpResponse {
    if user.pass.len() > 16
        || !PASSWD_REGEX.is_match(&user.pass).unwrap()
        || !EMAIL_REGEX.is_match(&user.email).unwrap()
        || !ID_REGEX.is_match(&user.user).unwrap()
    {
        HttpResponse::Ok().json(Action::Register {
            status: -10,
            id: Uuid::nil(),
        })
    } else {
        if handler::exists_user(&user.user, &user.email).await {
            HttpResponse::Ok().json(Action::Register {
                status: -1,
                id: Uuid::nil(),
            })
        } else {
            if let Ok(uuid) = handler::create_user(&user.user, &user.email, &user.pass).await {
                HttpResponse::Ok().json(Action::Register {
                    status: 0,
                    id: uuid,
                })
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}

#[post("/api/auth/getToken")]
async fn login(id: Identity, user: web::Form<LoginInfo>) -> HttpResponse {
    if user.key.len() > 16
        || !EMAIL_REGEX.is_match(&user.email).unwrap()
        || !PASSWD_REGEX.is_match(&user.key).unwrap()
    {
        HttpResponse::Ok().json(Action::GetToken { status: -10 })
    } else {
        if let Ok(result) = handler::check_password_email(&user.email, &user.key).await {
            if let Some(uuid) = result {
                id.remember(uuid.to_simple().to_string());
                HttpResponse::Ok().json(Action::GetToken { status: 0 })
            } else {
                HttpResponse::Ok().json(Action::GetToken { status: -1 })
            }
        } else {
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/api/auth/destroyToken")]
async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().json(Action::DestroyToken { status: 0 })
}

#[post("/api/auth/changePassword")]
async fn change_password(id: Identity, passwd: web::Form<ChangePassword>) -> HttpResponse {
    if let Some(uuid) = id.identity() {
        if passwd.old_pass.len() > 16
            || passwd.new_pass.len() > 16
            || !PASSWD_REGEX.is_match(&passwd.old_pass).unwrap()
            || !PASSWD_REGEX.is_match(&passwd.new_pass).unwrap()
        {
            HttpResponse::Ok().json(Action::ChangePassword { status: -10 })
        } else {
            if let Ok(opt) =
                handler::check_password_uuid(&Uuid::parse_str(&uuid).unwrap(), &passwd.old_pass)
                    .await
            {
                if opt.is_some() {
                    if let Ok(_) = handler::update_password(&uuid, &passwd.new_pass).await {
                        HttpResponse::Ok().json(Action::ChangePassword { status: 0 })
                    } else {
                        HttpResponse::Ok().json(Action::ChangePassword { status: -2 })
                    }
                } else {
                    HttpResponse::Ok().json(Action::ChangePassword { status: -1 })
                }
            } else {
                HttpResponse::Ok().json(Action::ChangePassword { status: -2 })
            }
        }
    } else {
        HttpResponse::Forbidden().finish()
    }
}

#[derive(Default, serde::Deserialize)]
#[serde(default)]
struct NginxRtmpForm {
    name: String,
    token: Uuid,
}

#[get("/callback/nginx")]
async fn nginx_callback(data: web::Query<NginxRtmpForm>) -> HttpResponse {
    if !ID_REGEX.is_match(&data.name).unwrap() || data.token.is_nil() {
        HttpResponse::Forbidden().body("Illegal Parameters!")
    } else {
        if let Ok(raw) = handler::raw_room_by_stream_name(&data.name).await {
            if let Some(raw) = raw {
                if raw.is_open() {
                    if raw.token_match(&data.token) {
                        HttpResponse::Ok().finish()
                    } else {
                        HttpResponse::Forbidden().body("Token Mismatched!")
                    }
                } else {
                    HttpResponse::Forbidden().body("Room is Not Open!")
                }
            } else {
                HttpResponse::Forbidden().body("Unknown Stream Name!")
            }
        } else {
            HttpResponse::InternalServerError().body("Sever Error Occurred!")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_room_list)
        .service(get_room_info)
        .service(get_user_list)
        .service(get_user_detail)
        .service(get_user_rooms)
        .service(user_room_detail)
        .service(create_room)
        .service(delete_room)
        .service(open_room)
        .service(close_room)
        .service(register)
        .service(login)
        .service(logout)
        .service(change_password)
        .service(nginx_callback);
}
