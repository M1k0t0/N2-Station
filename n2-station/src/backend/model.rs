pub mod form {
    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    pub struct RoomInfo {
        id: String,
    }

    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    pub struct TagInfo {
        id: String,
        #[serde(rename(deserialize = "type"))]
        tag_type: String,
    }

    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    pub struct UserInfo {
        id: String,
        name: String,
    }

    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    pub struct TagCreation {
        #[serde(rename(deserialize = "type"))]
        tag_type: String,
    }

    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    pub struct RoomCreation {
        id: String,
        title: String,
        desc: String,
        image: String,
        tag: Vec<String>,
    }

    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    pub struct RoomId {
        id: String,
    }

    pub type RoomEdition = RoomCreation;

    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    pub struct RegInfo {
        email: String,
        user: String,
        pass: String,
    }

    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    pub struct LoginInfo {
        email: String,
        user: String,
        uuid: String,
        key: String,
    }

    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    pub struct ChangePassword {
        #[serde(rename(deserialize = "oldPass"))]
        old_pass: String,
        #[serde(rename(deserialize = "newPass"))]
        new_pass: String,
    }
}

pub mod response {
    use actix_web::{Error, HttpResponse, Responder};
    use futures::future::{ready, Ready};
    use sqlx::MySqlPool;
    use time::OffsetDateTime;
    use uuid::Uuid;

    pub struct RawRoom {
        owner_uuid: Uuid,
        stream_id: String,
        title: String,
        desc: String,
        tag: String,
        open: bool,
    }

    #[derive(serde::Serialize)]
    pub struct User {
        id: Uuid,
        name: String,
    }

    #[derive(serde::Serialize)]
    pub struct BakedRoom {
        owner_uuid: Uuid,
        stream_id: String,
        title: String,
        desc: String,
        tag: Vec<String>,
        status: String,
        user: User,
    }

    #[derive(serde::Serialize)]
    pub struct Tag {
        id: u32,
        #[serde(rename = "type")]
        tag_type: String,
        creator_uuid: Uuid,
    }

    pub struct RawUser {
        uuid: Uuid,
        username: String,
        email: String,
        passwd: Vec<u8>,
        reg_date: OffsetDateTime,
    }

    #[derive(serde::Serialize)]
    pub struct BakedUser {
        id: Uuid,
        name: String,
        email: String,
    }

    #[derive(serde::Serialize)]
    #[serde(rename = "action")]
    #[serde(rename_all = "camelCase")]
    pub enum Action {
        GetRoomList(Vec<BakedRoom>),
        SearchRoom(Option<BakedRoom>),
        GetTagList(Vec<Tag>),
        SearchTag(Option<Tag>),
        GetUserList(Vec<BakedUser>),
        SearchUser(Option<BakedUser>),
        GetUserRoomList(Vec<BakedRoom>),
        CreateTag { status: i32 },
        CreateRoom { status: i32 },
        DeleteRoom { status: i32 },
        OpenRoom { status: i32 },
        CloseRoom { status: i32 },
        EditRoom(BakedRoom),
        Register { status: i32, id: Uuid },
        GetToken { status: i32 },
        DestroyToken { status: i32 },
        ChangePassword { status: i32 },
    }

    impl Responder for Action {
        type Error = Error;

        type Future = Ready<Result<HttpResponse, Error>>;

        fn respond_to(self, _req: &actix_web::HttpRequest) -> Self::Future {
            let body = serde_json::to_string(&self).unwrap();
            ready(Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(body)))
        }
    }

    impl RawRoom {
        pub fn bake(&self, db_pool: &MySqlPool) -> BakedRoom {
            todo!("bake the room")
            BakedRoom {
                owner_uuid: self.owner_uuid,
                stream_id: self.stream_id,
                title: self.title,
                desc: self.desc,
                tag: (),
                status: (),
                user: (),
            }
        }
    }
}

struct Backend;
