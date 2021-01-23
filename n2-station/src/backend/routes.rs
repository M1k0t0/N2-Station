use super::form;
use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse, Responder};
use form::{ChangePassword, LoginInfo, RegInfo};
use sqlx::MySqlPool;

#[get("/api/info/room")]
async fn get_room_list(db_pool: web::Data<MySqlPool>) -> impl Responder {
    ""
}

#[post("/api/info/room")]
async fn get_room_detail(
    db_pool: web::Data<MySqlPool>,
    room: web::Form<form::RoomInfo>,
) -> impl Responder {
    ""
}

#[get("/api/info/tag")]
async fn get_tag_list(db_pool: web::Data<MySqlPool>) -> impl Responder {
    ""
}

#[post("/api/info/tag")]
async fn get_tag_detail(
    db_pool: web::Data<MySqlPool>,
    tag: web::Form<form::TagInfo>,
) -> impl Responder {
    ""
}

#[get("/api/info/user")]
async fn get_user_list(db_pool: web::Data<MySqlPool>) -> impl Responder {
    ""
}

#[post("/api/info/user")]
async fn get_user_detail(
    db_pool: web::Data<MySqlPool>,
    user: web::Form<form::UserInfo>,
) -> impl Responder {
    ""
}

#[get("/api/user/room")]
async fn get_user_rooms(db_pool: web::Data<MySqlPool>, id: Identity) -> HttpResponse {
    if id.identity().is_none() {
        return HttpResponse::Forbidden().finish();
    } else {
        return HttpResponse::Found().finish();
    }
}

#[post("/api/user/createTag")]
async fn create_tag(
    db_pool: web::Data<MySqlPool>,
    id: Identity,
    creation: web::Form<form::TagCreation>,
) -> HttpResponse {
    if id.identity().is_none() {
        return HttpResponse::Forbidden().finish();
    } else {
        return HttpResponse::Found().finish();
    }
}

#[post("/api/user/createRoom")]
async fn create_room(
    db_pool: web::Data<MySqlPool>,
    id: Identity,
    room: web::Form<form::RoomId>,
) -> HttpResponse {
    if id.identity().is_none() {
        return HttpResponse::Forbidden().finish();
    } else {
        return HttpResponse::Found().finish();
    }
}

#[post("/api/user/deleteRoom")]
async fn delete_room(
    db_pool: web::Data<MySqlPool>,
    id: Identity,
    room: web::Form<form::RoomId>,
) -> HttpResponse {
    if id.identity().is_none() {
        return HttpResponse::Forbidden().finish();
    } else {
        return HttpResponse::Found().finish();
    }
}

#[post("/api/user/openRoom")]
async fn open_room(
    db_pool: web::Data<MySqlPool>,
    id: Identity,
    room: web::Form<form::RoomId>,
) -> HttpResponse {
    if id.identity().is_none() {
        return HttpResponse::Forbidden().finish();
    } else {
        return HttpResponse::Found().finish();
    }
}

#[post("/api/user/closeRoom")]
async fn close_room(
    db_pool: web::Data<MySqlPool>,
    id: Identity,
    room: web::Form<form::RoomId>,
) -> HttpResponse {
    if id.identity().is_none() {
        return HttpResponse::Forbidden().finish();
    } else {
        return HttpResponse::Found().finish();
    }
}

#[post("/api/user/editRoom")]
async fn edit_room(
    db_pool: web::Data<MySqlPool>,
    id: Identity,
    room: web::Form<form::RoomEdition>,
) -> HttpResponse {
    if id.identity().is_none() {
        return HttpResponse::Forbidden().finish();
    } else {
        return HttpResponse::Found().finish();
    }
}

#[post("/api/auth/register")]
async fn register(db_pool: web::Data<MySqlPool>, user: web::Form<RegInfo>) -> impl Responder {
    ""
}

#[post("/api/auth/getToken")]
async fn login(
    db_pool: web::Data<MySqlPool>,
    id: Identity,
    user: web::Form<LoginInfo>,
) -> impl Responder {
    //verify
    id.remember(String::from("uuid"));
    ""
}

#[get("/api/auth/destroyToken")]
async fn logout(db_pool: web::Data<MySqlPool>, id: Identity) -> impl Responder {
    id.forget();
    ""
}

#[post("/api/auth/changePassword")]
async fn change_password(
    db_pool: web::Data<MySqlPool>,
    id: Identity,
    user: web::Form<ChangePassword>,
) -> impl Responder {
    ""
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_room_list)
        .service(get_room_detail)
        .service(get_tag_list)
        .service(get_tag_detail)
        .service(get_user_list)
        .service(get_user_detail)
        .service(get_user_rooms)
        .service(create_tag)
        .service(create_room)
        .service(delete_room)
        .service(open_room)
        .service(close_room)
        .service(edit_room)
        .service(register)
        .service(login)
        .service(logout)
        .service(change_password);
}
