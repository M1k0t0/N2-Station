pub mod form {
    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    pub struct RoomInfo {
        pub id: String,
    }

    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    pub struct TagInfo {
        pub id: Option<u32>,
        #[serde(rename(deserialize = "type"))]
        pub tag_type: String,
    }

    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    pub struct UserInfo {
        pub id: String,
        pub name: String,
    }

    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    pub struct TagCreation {
        #[serde(rename(deserialize = "type"))]
        pub tag_type: String,
    }

    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    pub struct RoomCreation {
        pub id: String,
        pub title: String,
        pub desc: String,
        pub image: String,
        pub tag: Vec<String>,
    }

    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    pub struct RoomId {
        pub id: String,
    }

    pub type RoomEdition = RoomCreation;

    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    pub struct RegInfo {
        pub email: String,
        pub user: String,
        pub pass: String,
    }

    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    pub struct LoginInfo {
        pub email: String,
        pub user: String,
        pub uuid: String,
        pub key: String,
    }

    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    pub struct ChangePassword {
        #[serde(rename(deserialize = "oldPass"))]
        pub old_pass: String,
        #[serde(rename(deserialize = "newPass"))]
        pub new_pass: String,
    }
}

pub mod response {
    use anyhow::Result;
    use sqlx::{query_as, MySqlPool};
    use time::OffsetDateTime;
    use uuid::Uuid;

    pub struct RawRoom {
        pub(super) owner_uuid: String,
        pub(super) stream_id: String,
        pub(super) title: String,
        pub(super) desc: String,
        pub(super) tag: Option<String>,
        pub(super) open: i8,
        pub(super) stream_token: Option<String>,
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

    pub struct RawTag {
        pub(super) id: u32,
        pub(super) tag_type: String,
        pub(super) creator_uuid: String,
    }

    #[derive(serde::Serialize)]
    pub struct BakedTag {
        id: u32,
        #[serde(rename = "type")]
        tag_type: String,
        creator_uuid: Uuid,
    }

    pub struct RawUser {
        pub(super) uuid: String,
        pub(super) username: String,
        pub(super) email: String,
        pub(super) passwd: Vec<u8>,
        pub(super) reg_date: OffsetDateTime,
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
        GetTagList(Vec<BakedTag>),
        SearchTag(Option<BakedTag>),
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

    //Seems to be useless now
    /*
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
    */

    impl RawRoom {
        pub async fn bake(&self, db_pool: &MySqlPool) -> Result<BakedRoom> {
            let tag = self
                .tag
                .clone()
                .unwrap_or(String::from(""))
                .split(';')
                .map(|s| s.to_string())
                .collect();
            let status = if self.open == 1 { "open" } else { "close" };
            let uuid = Uuid::parse_str(self.owner_uuid.as_str())?;
            let raw = query_as!(
                RawUser,
                r#"SELECT * FROM users where uuid = ?"#,
                self.owner_uuid
            )
            .fetch_one(db_pool)
            .await?;
            Ok(BakedRoom {
                owner_uuid: uuid,
                stream_id: self.stream_id.clone(),
                title: self.title.clone(),
                desc: self.desc.clone(),
                tag: tag,
                status: status.to_string(),
                user: raw.into(),
            })
        }
    }

    impl RawUser {
        pub fn bake(&self) -> BakedUser {
            BakedUser {
                id: Uuid::parse_str(self.uuid.as_str()).unwrap(),
                name: self.username.clone(),
                email: self.email.clone(),
            }
        }
    }

    impl From<RawUser> for User {
        fn from(raw: RawUser) -> Self {
            Self {
                id: Uuid::parse_str(raw.uuid.as_str()).unwrap(),
                name: raw.username,
            }
        }
    }

    impl RawTag {
        pub fn bake(&self) -> BakedTag {
            BakedTag {
                id: self.id,
                tag_type: self.tag_type.clone(),
                creator_uuid: Uuid::parse_str(self.creator_uuid.as_str()).unwrap(),
            }
        }
    }
}

pub mod handler {
    use super::response::{BakedRoom, BakedTag, BakedUser, RawRoom, RawTag, RawUser};
    use anyhow::Result;
    use futures::{stream, StreamExt};
    use sqlx::{query, query_as, Done, MySqlPool};
    use uuid::Uuid;

    pub async fn get_all_rooms(db_pool: &MySqlPool) -> Result<Vec<BakedRoom>> {
        Ok(stream::iter(
            query_as!(RawRoom, r#"SELECT * FROM rooms"#)
                .fetch_all(db_pool)
                .await?,
        )
        .then(|raw| async move { raw.bake(db_pool).await.unwrap() })
        .collect()
        .await)
    }

    pub async fn search_rooms_by_owner(
        db_pool: &MySqlPool,
        owner: String,
    ) -> Result<Vec<BakedRoom>> {
        Ok(stream::iter(
            query_as!(
                RawRoom,
                r#"SELECT * FROM rooms WHERE owner_uuid = ?"#,
                owner
            )
            .fetch_all(db_pool)
            .await?,
        )
        .then(|raw| async move { raw.bake(db_pool).await.unwrap() })
        .collect()
        .await)
    }

    pub async fn search_room_by_stream_name(
        db_pool: &MySqlPool,
        stream_name: String,
    ) -> Result<Option<BakedRoom>> {
        let query = query_as!(
            RawRoom,
            r#"SELECT * FROM rooms WHERE stream_id = ?"#,
            stream_name
        )
        .fetch_optional(db_pool)
        .await?;
        if let Some(raw) = query {
            Ok(Some(raw.bake(db_pool).await?))
        } else {
            Ok(None)
        }
    }

    pub async fn get_all_tags(db_pool: &MySqlPool) -> Result<Vec<BakedTag>> {
        Ok(query_as!(RawTag, r#"SELECT * FROM tags"#)
            .fetch_all(db_pool)
            .await?
            .iter()
            .map(RawTag::bake)
            .collect())
    }

    pub async fn search_tag_by_id(db_pool: &MySqlPool, id: u32) -> Result<Option<BakedTag>> {
        let query = query_as!(RawTag, r#"SELECT * FROM tags WHERE id = ?"#, id)
            .fetch_optional(db_pool)
            .await?;
        if let Some(raw) = query {
            Ok(Some(raw.bake()))
        } else {
            Ok(None)
        }
    }

    pub async fn search_tag_by_type(
        db_pool: &MySqlPool,
        tag_type: String,
    ) -> Result<Option<BakedTag>> {
        let query = query_as!(RawTag, r#"SELECT * FROM tags WHERE tag_type = ?"#, tag_type)
            .fetch_optional(db_pool)
            .await?;
        if let Some(raw) = query {
            Ok(Some(raw.bake()))
        } else {
            Ok(None)
        }
    }

    pub async fn get_all_users(db_pool: &MySqlPool) -> Result<Vec<BakedUser>> {
        Ok(query_as!(RawUser, r#"SELECT * FROM users"#)
            .fetch_all(db_pool)
            .await?
            .iter()
            .map(RawUser::bake)
            .collect())
    }

    pub async fn search_user_by_uuid(db_pool: &MySqlPool, uuid: Uuid) -> Result<Option<BakedUser>> {
        let uuid = uuid.to_simple().to_string();
        let query = query_as!(RawUser, r#"SELECT * FROM users WHERE uuid = ?"#, uuid)
            .fetch_optional(db_pool)
            .await?;
        if let Some(user) = query {
            Ok(Some(user.bake()))
        } else {
            Ok(None)
        }
    }

    pub async fn search_user_by_name(
        db_pool: &MySqlPool,
        name: String,
    ) -> Result<Option<BakedUser>> {
        let query = query_as!(RawUser, r#"SELECT * FROM users WHERE username = ?"#, name)
            .fetch_optional(db_pool)
            .await?;
        if let Some(user) = query {
            Ok(Some(user.bake()))
        } else {
            Ok(None)
        }
    }

    pub async fn create_tag(db_pool: &MySqlPool, tag_type: String, owner: String) -> Result<()> {
        query!(
            r#"INSERT INTO tags (tag_type, creator_uuid) VALUES(?, ?)"#,
            tag_type,
            owner
        )
        .execute(db_pool)
        .await?;
        Ok(())
    }

    pub async fn create_room(
        db_pool: &MySqlPool,
        creator: String,
        stream_id: String,
        title: String,
        desc: String,
        tag: Vec<String>,
    ) -> Result<()> {
        let tag = if tag.len() > 0 {
            Some(tag.join(";"))
        } else {
            None
        };
        query!(
            r#"INSERT INTO rooms (owner_uuid, stream_id, title, `desc`, tag, open) VALUES(?, ?, ?, ?, ?, FALSE)"#,
            creator,
            stream_id,
            title,
            desc,
            tag
        )
        .execute(db_pool)
        .await?;
        Ok(())
    }

    pub async fn delete_room(db_pool: &MySqlPool, stream_id: String) -> Result<u64> {
        Ok(
            query!(r#"DELETE FROM rooms WHERE stream_id = ?"#, stream_id)
                .execute(db_pool)
                .await?
                .rows_affected(),
        )
    }

    pub async fn exists_room(db_pool: &MySqlPool, stream_id: String) -> bool {
        query!(r#"SELECT * FROM rooms WHERE stream_id = ?"#, stream_id)
            .fetch_optional(db_pool)
            .await
            .unwrap()
            .is_some()
    }

    pub async fn exists_user(db_pool: &MySqlPool, username: String, email: String) -> bool {
        query!(
            r#"SELECT * FROM users WHERE username = ? OR email = ?"#,
            username,
            email
        )
        .fetch_optional(db_pool)
        .await
        .unwrap()
        .is_some()
    }

    pub async fn set_room_status(
        db_pool: &MySqlPool,
        stream_id: String,
        open: bool,
    ) -> Result<u64> {
        Ok(query!(
            r#"UPDATE rooms SET open = ? WHERE stream_id = ?"#,
            open,
            stream_id
        )
        .execute(db_pool)
        .await?
        .rows_affected())
    }
}
