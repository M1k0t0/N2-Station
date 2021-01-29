//!TODO: more appropriate error handling

use crate::ServerConfig;

use super::{form, handler, Action};
use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse};
use form::{ChangePassword, LoginInfo, RegInfo};
use sqlx::MySqlPool;
use uuid::Uuid;

#[get("/api/info/room")]
async fn get_room_list(db_pool: web::Data<MySqlPool>) -> HttpResponse {
    let result = handler::get_all_rooms(db_pool.get_ref()).await;
    if let Ok(rooms) = result {
        HttpResponse::Ok().json(Action::GetRoomList { rooms })
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[post("/api/info/room")]
async fn get_room_info(
    db_pool: web::Data<MySqlPool>,
    room: web::Form<form::RoomInfo>,
) -> HttpResponse {
    let result = handler::search_room_by_stream_name(db_pool.get_ref(), &room.id, "", false).await;
    if let Ok(room) = result {
        HttpResponse::Ok().json(Action::SearchRoom { room })
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[get("/api/info/user")]
async fn get_user_list(db_pool: web::Data<MySqlPool>) -> HttpResponse {
    let result = handler::get_all_users(db_pool.get_ref()).await;
    if let Ok(users) = result {
        HttpResponse::Ok().json(Action::GetUserList { users })
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[post("/api/info/user")]
async fn get_user_detail(
    db_pool: web::Data<MySqlPool>,
    user: web::Form<form::UserInfo>,
) -> HttpResponse {
    let result = if !user.id.is_empty() {
        if let Ok(uuid) = Uuid::parse_str(user.id.as_str()) {
            handler::search_user_by_uuid(db_pool.get_ref(), uuid).await
        } else {
            return HttpResponse::BadRequest().finish();
        }
    } else if !user.name.is_empty() {
        handler::search_user_by_name(db_pool.get_ref(), &user.name).await
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
async fn get_user_rooms(db_pool: web::Data<MySqlPool>, id: Identity) -> HttpResponse {
    if let Some(uuid) = id.identity() {
        let result = handler::search_rooms_by_owner(db_pool.get_ref(), &uuid).await;
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
async fn user_room_detail(
    db_pool: web::Data<MySqlPool>,
    id: Identity,
    room: web::Form<form::RoomId>,
) -> HttpResponse {
    if let Some(uuid) = id.identity() {
        let result =
            handler::search_room_by_stream_name(db_pool.get_ref(), &room.id, &uuid, true).await;
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
    db_pool: web::Data<MySqlPool>,
    config: web::Data<ServerConfig>,
    id: Identity,
    room: web::Form<form::RoomCreation>,
) -> HttpResponse {
    if let Some(uuid) = id.identity() {
        if handler::raw_rooms_for_user(db_pool.get_ref(), &uuid)
            .await
            .unwrap()
            .len()
            > config.room_creation_limit
        {
            HttpResponse::Ok().json(Action::OpenRoom {
                stream_token: Uuid::nil(),
                status: -2,
            })
        } else if room.id.is_empty()
            || room.title.is_empty()
            || room.desc.is_empty()
            || room.id.len() > 16
            || room.title.len() > 16
            || room.desc.len() > 20
            || room.tag.iter().any(|s| s.contains(';'))
        {
            HttpResponse::Ok().json(Action::CreateRoom { status: -10 })
        } else {
            if handler::exists_room(db_pool.get_ref(), &room.id).await {
                HttpResponse::Ok().json(Action::CreateRoom { status: -1 })
            } else {
                if let Ok(_) = handler::create_room(
                    db_pool.get_ref(),
                    &uuid,
                    &room.id,
                    &room.title,
                    &room.desc,
                    room.tag.clone(),
                )
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
async fn delete_room(
    db_pool: web::Data<MySqlPool>,
    id: Identity,
    room: web::Form<form::RoomId>,
) -> HttpResponse {
    if let Some(uuid) = id.identity() {
        if room.id.is_empty() {
            HttpResponse::Ok().json(Action::DeleteRoom { status: -10 })
        } else {
            if let Ok(affected) = handler::delete_room(db_pool.get_ref(), &room.id, &uuid).await {
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
    db_pool: web::Data<MySqlPool>,
    config: web::Data<ServerConfig>,
    id: Identity,
    room: web::Form<form::RoomId>,
) -> HttpResponse {
    if let Some(uuid) = id.identity() {
        if handler::raw_open_rooms(db_pool.get_ref())
            .await
            .unwrap()
            .len()
            > config.room_open_limit
        {
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
            if let Ok(affected) =
                handler::set_room_status(db_pool.get_ref(), &room.id, true, &uuid).await
            {
                if affected > 0 {
                    let uuid = handler::assign_stream_token(db_pool.get_ref(), &room.id)
                        .await
                        .unwrap();
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
async fn close_room(
    db_pool: web::Data<MySqlPool>,
    id: Identity,
    room: web::Form<form::RoomId>,
) -> HttpResponse {
    if let Some(uuid) = id.identity() {
        if room.id.is_empty() {
            HttpResponse::Ok().json(Action::CloseRoom { status: -10 })
        } else {
            if let Ok(affected) =
                handler::set_room_status(db_pool.get_ref(), &room.id, false, &uuid).await
            {
                if affected > 0 {
                    handler::unset_stream_token(db_pool.get_ref(), &room.id)
                        .await
                        .unwrap();
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
async fn register(db_pool: web::Data<MySqlPool>, user: web::Form<RegInfo>) -> HttpResponse {
    if user.email.is_empty()
        || user.pass.is_empty()
        || user.user.is_empty()
        || user.user.len() > 16
        || user.email.len() > 30
        || user.pass.len() > 16
    {
        HttpResponse::Ok().json(Action::Register {
            status: -10,
            id: Uuid::nil(),
        })
    } else {
        if handler::exists_user(db_pool.get_ref(), &user.user, &user.email).await {
            HttpResponse::Ok().json(Action::Register {
                status: -1,
                id: Uuid::nil(),
            })
        } else {
            if let Ok(uuid) =
                handler::create_user(db_pool.get_ref(), &user.user, &user.email, &user.pass).await
            {
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
async fn login(
    db_pool: web::Data<MySqlPool>,
    id: Identity,
    user: web::Form<LoginInfo>,
) -> HttpResponse {
    if user.email.is_empty() || user.key.is_empty() || user.email.len() > 30 || user.key.len() > 16
    {
        HttpResponse::Ok().json(Action::GetToken { status: -10 })
    } else {
        if let Ok(result) =
            handler::check_password_email(db_pool.get_ref(), &user.email, &user.key).await
        {
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
async fn change_password(
    db_pool: web::Data<MySqlPool>,
    id: Identity,
    passwd: web::Form<ChangePassword>,
) -> HttpResponse {
    if let Some(uuid) = id.identity() {
        if passwd.new_pass.is_empty()
            || passwd.old_pass.is_empty()
            || passwd.new_pass.len() > 16
            || passwd.old_pass.len() > 16
        {
            HttpResponse::Ok().json(Action::ChangePassword { status: -10 })
        } else {
            if let Ok(opt) = handler::check_password_uuid(
                db_pool.get_ref(),
                &Uuid::parse_str(&uuid).unwrap(),
                &passwd.old_pass,
            )
            .await
            {
                if opt.is_some() {
                    if let Ok(_) =
                        handler::update_password(db_pool.get_ref(), &uuid, &passwd.new_pass).await
                    {
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
async fn nginx_callback(
    db_pool: web::Data<MySqlPool>,
    data: web::Query<NginxRtmpForm>,
) -> HttpResponse {
    if data.name.is_empty() || data.token.is_nil() || data.name.len() > 16 {
        HttpResponse::Forbidden().body("Illegal Parameters!")
    } else {
        if let Ok(raw) = handler::raw_room_by_stream_name(db_pool.get_ref(), &data.name).await {
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