//!TODO: more appropriate error handling

use actix_web::web;
use fancy_regex::Regex;
use lazy_static::lazy_static;
use rbatis::rbatis::Rbatis;
lazy_static! {
    pub static ref ID_REGEX: Regex = Regex::new("^[A-Za-z0-9_-]{4,16}$").unwrap();
    pub static ref EMAIL_REGEX: Regex = Regex::new("^[a-zA-Z0-9_-]+@[a-zA-Z0-9_-]+(\\.[a-zA-Z0-9_-]+)+$").unwrap();
    pub static ref PASSWD_REGEX: Regex = Regex::new("^(?![0-9]+$)(?![a-zA-Z]+$)(?![0-9a-zA-Z]+$)(?![0-9\\W]+$)(?![a-zA-Z\\W]+$)[0-9A-Za-z\\W]{8,16}$").unwrap();
    pub static ref TITLE_REGEX: Regex = Regex::new("^.{1,16}$").unwrap();
    pub static ref DESC_REGEX: Regex = Regex::new("^.{1,20}$").unwrap();

    pub static ref RBATIS: Rbatis = Rbatis::new();
}

mod api {

    pub use super::{DESC_REGEX, EMAIL_REGEX, ID_REGEX, PASSWD_REGEX, TITLE_REGEX};

    pub mod info {
        use actix_web::{get, post, web, HttpResponse};
        use uuid::Uuid;

        use crate::backend::{form, handler, Action};

        #[get("/room")]
        async fn get_room_list() -> HttpResponse {
            let result = handler::get_all_rooms().await;
            if let Ok(rooms) = result {
                HttpResponse::Ok().json(Action::GetRoomList { rooms })
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }

        #[post("/room")]
        async fn get_room_info(room: web::Form<form::RoomInfo>) -> HttpResponse {
            let result = handler::search_room_by_stream_name(&room.id, "", false).await;
            if let Ok(room) = result {
                HttpResponse::Ok().json(Action::SearchRoom { room })
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }

        #[get("/user")]
        async fn get_user_list() -> HttpResponse {
            let result = handler::get_all_users().await;
            if let Ok(users) = result {
                HttpResponse::Ok().json(Action::GetUserList { users })
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }

        #[post("/user")]
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
    }

    pub mod user {
        use super::{DESC_REGEX, ID_REGEX, TITLE_REGEX};

        use actix::Addr;
        use actix_identity::Identity;
        use actix_web::{get, post, web, HttpResponse};
        use uuid::Uuid;

        use crate::{
            backend::{
                danmaku::{CloseRoom, OpenRoom},
                form, handler, Action, ChatServer, IncrementalServer,
            },
            ServerConfig,
        };

        #[get("/room")]
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

        #[post("/room")]
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

        #[post("/createRoom")]
        async fn create_room(
            config: web::Data<ServerConfig>,
            id: Identity,
            room: web::Form<form::RoomCreation>,
            svr: web::Data<Addr<IncrementalServer>>,
        ) -> HttpResponse {
            if let Some(uuid) = id.identity() {
                if handler::raw_rooms_for_user(&uuid).await.unwrap() > config.room_creation_limit {
                    HttpResponse::Ok().json(Action::CreateRoom { status: -2 })
                } else if !ID_REGEX.is_match(&room.id).unwrap()
                    || !TITLE_REGEX.is_match(&room.title).unwrap()
                    || !DESC_REGEX.is_match(&room.desc).unwrap()
                    || room.tag.contains(';')
                {
                    HttpResponse::Ok().json(Action::CreateRoom { status: -10 })
                } else {
                    if handler::exists_room(&room.id).await {
                        HttpResponse::Ok().json(Action::CreateRoom { status: -1 })
                    } else {
                        if let Ok(_) = handler::create_room(
                            &uuid,
                            &room.id,
                            &room.title,
                            &room.desc,
                            &room.tag,
                            &room.icon,
                            svr.get_ref(),
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

        #[post("/deleteRoom")]
        async fn delete_room(
            id: Identity,
            room: web::Form<form::RoomId>,
            svr: web::Data<Addr<IncrementalServer>>,
        ) -> HttpResponse {
            if let Some(uuid) = id.identity() {
                if room.id.is_empty() {
                    HttpResponse::Ok().json(Action::DeleteRoom { status: -10 })
                } else {
                    if let Ok(affected) = handler::delete_room(&room.id, &uuid, svr.get_ref()).await
                    {
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

        #[post("/openRoom")]
        async fn open_room(
            config: web::Data<ServerConfig>,
            id: Identity,
            room: web::Form<form::RoomId>,
            chat_svr: web::Data<Addr<ChatServer>>,
            inc_svr: web::Data<Addr<IncrementalServer>>,
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
                    if let Ok(affected) =
                        handler::set_room_status(&room.id, true, &uuid, inc_svr.get_ref()).await
                    {
                        if affected > 0 {
                            chat_svr.get_ref().do_send(OpenRoom {
                                room: room.id.clone(),
                            });
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

        #[post("/closeRoom")]
        async fn close_room(
            id: Identity,
            room: web::Form<form::RoomId>,
            chat_svr: web::Data<Addr<ChatServer>>,
            inc_svr: web::Data<Addr<IncrementalServer>>,
        ) -> HttpResponse {
            if let Some(uuid) = id.identity() {
                if room.id.is_empty() {
                    HttpResponse::Ok().json(Action::CloseRoom { status: -10 })
                } else {
                    if let Ok(affected) =
                        handler::set_room_status(&room.id, false, &uuid, inc_svr.get_ref()).await
                    {
                        if affected > 0 {
                            chat_svr.get_ref().do_send(CloseRoom {
                                room: room.id.clone(),
                            });
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
    }

    pub mod auth {
        use super::{EMAIL_REGEX, ID_REGEX, PASSWD_REGEX};

        use actix_identity::Identity;
        use actix_web::{get, post, web, HttpResponse};
        use uuid::Uuid;

        use crate::backend::{
            form::{ChangePassword, LoginInfo, RegInfo},
            handler, Action,
        };

        #[post("/register")]
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
                    if let Ok(uuid) =
                        handler::create_user(&user.user, &user.email, &user.pass).await
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

        #[post("/getToken")]
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

        #[get("/destroyToken")]
        async fn logout(id: Identity) -> HttpResponse {
            id.forget();
            HttpResponse::Ok().json(Action::DestroyToken { status: 0 })
        }

        #[post("/changePassword")]
        async fn change_password(id: Identity, passwd: web::Form<ChangePassword>) -> HttpResponse {
            if let Some(uuid) = id.identity() {
                if passwd.old_pass.len() > 16
                    || passwd.new_pass.len() > 16
                    || !PASSWD_REGEX.is_match(&passwd.old_pass).unwrap()
                    || !PASSWD_REGEX.is_match(&passwd.new_pass).unwrap()
                {
                    HttpResponse::Ok().json(Action::ChangePassword { status: -10 })
                } else {
                    if let Ok(opt) = handler::check_password_uuid(
                        &Uuid::parse_str(&uuid).unwrap(),
                        &passwd.old_pass,
                    )
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
    }
}

pub mod callback {
    use super::ID_REGEX;

    use actix_web::{get, web, HttpResponse};
    use uuid::Uuid;

    use crate::backend::handler;

    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    struct NginxRtmpForm {
        name: String,
        token: Uuid,
    }

    #[get("/nginx")]
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
}

pub mod chat {
    use actix::Addr;
    use actix_identity::Identity;
    use actix_web::{get, web, Error, HttpRequest, HttpResponse};
    use actix_web_actors::ws;
    use uuid::Uuid;

    use crate::backend::{danmaku::DanmakuSession, handler, ChatServer};

    #[get("/{room}")]
    async fn upgrade_ws(
        req: HttpRequest,
        id: Identity,
        path: web::Path<(String,)>,
        stream: web::Payload,
        svr: web::Data<Addr<ChatServer>>,
    ) -> Result<HttpResponse, Error> {
        let (room,) = path.into_inner();
        if let Ok(raw) = handler::raw_room_by_stream_name(&room).await {
            if let Some(raw) = raw {
                if raw.is_open() {
                    if let Some(uuid) = id.identity() {
                        if let Ok(Some(user)) =
                            handler::search_user_by_uuid(Uuid::parse_str(&uuid).unwrap()).await
                        {
                            ws::start(
                                DanmakuSession::new(
                                    &room,
                                    Some(&user.name()),
                                    svr.get_ref().clone(),
                                ),
                                &req,
                                stream,
                            )
                        } else {
                            Ok(HttpResponse::InternalServerError().finish())
                        }
                    } else {
                        ws::start(
                            DanmakuSession::new(&room, None, svr.get_ref().clone()),
                            &req,
                            stream,
                        )
                    }
                } else {
                    Ok(HttpResponse::Ok().body("Room is closed!"))
                }
            } else {
                Ok(HttpResponse::Ok().body("No such room exists!"))
            }
        } else {
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

pub mod subscribe {
    use actix::Addr;
    use actix_web::{get, web, Error, HttpRequest, HttpResponse};
    use actix_web_actors::ws;

    use crate::backend::{incremental::IncrementalSession, IncrementalServer};

    #[get("/subscribe")]
    async fn upgrade_ws(
        req: HttpRequest,
        stream: web::Payload,
        svr: web::Data<Addr<IncrementalServer>>,
    ) -> Result<HttpResponse, Error> {
        ws::start(IncrementalSession::new(svr.get_ref().clone()), &req, stream)
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/info")
                    .service(api::info::get_room_list)
                    .service(api::info::get_room_info)
                    .service(api::info::get_user_list)
                    .service(api::info::get_user_detail),
            )
            .service(
                web::scope("/user")
                    .service(api::user::get_user_rooms)
                    .service(api::user::user_room_detail)
                    .service(api::user::create_room)
                    .service(api::user::delete_room)
                    .service(api::user::open_room)
                    .service(api::user::close_room),
            )
            .service(
                web::scope("/auth")
                    .service(api::auth::register)
                    .service(api::auth::login)
                    .service(api::auth::logout)
                    .service(api::auth::change_password),
            ),
    )
    .service(web::scope("/callback").service(callback::nginx_callback))
    .service(web::scope("/chat").service(chat::upgrade_ws));
}
