pub mod form {
    use uuid::Uuid;

    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    pub struct RoomInfo {
        pub id: String,
    }

    #[derive(Default, serde::Deserialize)]
    #[serde(default)]
    pub struct UserInfo {
        pub id: String,
        pub name: String,
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

    #[derive(Default, serde::Deserialize)]
    pub struct RoomEdition {
        pub room: Uuid,
        pub update: RoomCreation,
    }

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

    #[derive(serde::Serialize, Default)]
    pub struct User {
        id: Uuid,
        name: String,
    }

    #[derive(serde::Serialize, Default)]
    pub struct BakedRoom {
        owner_uuid: Uuid,
        stream_id: String,
        title: String,
        desc: String,
        tag: Vec<String>,
        status: String,
        user: User,
        stream_token: Option<Uuid>,
    }

    pub struct RawUser {
        pub(super) uuid: String,
        pub(super) username: String,
        pub(super) email: String,
        pub(super) passwd: Vec<u8>,
        #[allow(dead_code)]
        pub(super) reg_date: OffsetDateTime,
    }

    #[derive(serde::Serialize)]
    pub struct BakedUser {
        id: Uuid,
        name: String,
        email: String,
    }

    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "action")]
    pub enum Action {
        GetRoomList { rooms: Vec<BakedRoom> },
        SearchRoom { room: Option<BakedRoom> },
        GetUserList { users: Vec<BakedUser> },
        SearchUser { user: Option<BakedUser> },
        GetUserRoomList { rooms: Vec<BakedRoom> },
        UserRoomDetail { room: Option<BakedRoom> },
        CreateRoom { status: i32 },
        DeleteRoom { status: i32 },
        OpenRoom { stream_token: Uuid, status: i32 },
        CloseRoom { status: i32 },
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
        pub async fn bake(&self, db_pool: &MySqlPool, detail: bool) -> Result<BakedRoom> {
            let tag = self
                .tag
                .clone()
                .unwrap_or(String::from(""))
                .split(';')
                .map(|s| s.to_string())
                .collect();
            let status = if self.is_open() { "open" } else { "close" };
            let uuid = Uuid::parse_str(self.owner_uuid.as_str())?;
            let raw = query_as!(
                RawUser,
                r#"SELECT * FROM users where uuid = ?"#,
                self.owner_uuid
            )
            .fetch_one(db_pool)
            .await?;
            let stream_token = if detail && self.is_open() {
                if let Some(ref uuid) = self.stream_token {
                    Some(Uuid::parse_str(uuid.as_str()).unwrap())
                } else {
                    None
                }
            } else {
                None
            };
            Ok(BakedRoom {
                owner_uuid: uuid,
                stream_id: self.stream_id.clone(),
                title: self.title.clone(),
                desc: self.desc.clone(),
                tag: tag,
                status: status.to_string(),
                user: raw.into(),
                stream_token,
            })
        }

        pub fn is_open(&self) -> bool {
            self.open != 0
        }

        pub fn token_match(&self, token: &Uuid) -> bool {
            if let Some(ref stream_token) = self.stream_token {
                stream_token.eq(&token.to_simple().to_string())
            } else {
                false
            }
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
}

pub mod handler {
    use super::response::{BakedRoom, BakedUser, RawRoom, RawUser};
    use anyhow::Result;
    use bcrypt::{hash, verify};
    use futures::{stream, StreamExt};
    use sqlx::{query, query_as, Done, MySqlPool};
    use uuid::Uuid;

    pub async fn get_all_rooms(db_pool: &MySqlPool) -> Result<Vec<BakedRoom>> {
        Ok(stream::iter(
            query_as!(RawRoom, r#"SELECT * FROM rooms"#)
                .fetch_all(db_pool)
                .await?,
        )
        .then(|raw| async move { raw.bake(db_pool, false).await.unwrap() })
        .collect()
        .await)
    }

    pub async fn search_rooms_by_owner(db_pool: &MySqlPool, owner: &str) -> Result<Vec<BakedRoom>> {
        Ok(stream::iter(
            query_as!(
                RawRoom,
                r#"SELECT * FROM rooms WHERE owner_uuid = ?"#,
                owner
            )
            .fetch_all(db_pool)
            .await?,
        )
        .then(|raw| async move { raw.bake(db_pool, false).await.unwrap() })
        .collect()
        .await)
    }

    pub async fn raw_room_by_stream_name(
        db_pool: &MySqlPool,
        stream_name: &str,
    ) -> Result<Option<RawRoom>> {
        Ok(query_as!(
            RawRoom,
            r#"SELECT * FROM rooms WHERE stream_id = ?"#,
            stream_name
        )
        .fetch_optional(db_pool)
        .await?)
    }

    pub async fn raw_rooms_for_user(db_pool: &MySqlPool, user: &str) -> Result<Vec<RawRoom>> {
        Ok(
            query_as!(RawRoom, r#"SELECT * FROM rooms WHERE owner_uuid = ?"#, user)
                .fetch_all(db_pool)
                .await?,
        )
    }

    pub async fn raw_open_rooms(db_pool: &MySqlPool) -> Result<Vec<RawRoom>> {
        Ok(
            query_as!(RawRoom, r#"SELECT * FROM rooms WHERE open = TRUE"#)
                .fetch_all(db_pool)
                .await?,
        )
    }

    pub async fn search_room_by_stream_name(
        db_pool: &MySqlPool,
        stream_name: &str,
        owner_uuid: &str,
        detail: bool,
    ) -> Result<Option<BakedRoom>> {
        let query = raw_room_by_stream_name(db_pool, stream_name).await?;
        if let Some(raw) = query {
            if detail && raw.owner_uuid == owner_uuid {
                Ok(Some(raw.bake(db_pool, detail).await?))
            } else {
                Ok(None)
            }
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

    pub async fn search_user_by_name(db_pool: &MySqlPool, name: &str) -> Result<Option<BakedUser>> {
        let query = query_as!(RawUser, r#"SELECT * FROM users WHERE username = ?"#, name)
            .fetch_optional(db_pool)
            .await?;
        if let Some(user) = query {
            Ok(Some(user.bake()))
        } else {
            Ok(None)
        }
    }

    pub async fn create_room(
        db_pool: &MySqlPool,
        creator: &str,
        stream_id: &str,
        title: &str,
        desc: &str,
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

    pub async fn delete_room(
        db_pool: &MySqlPool,
        stream_id: &str,
        owner_uuid: &str,
    ) -> Result<u64> {
        Ok(query!(
            r#"DELETE FROM rooms WHERE stream_id = ? AND owner_uuid = ?"#,
            stream_id,
            owner_uuid
        )
        .execute(db_pool)
        .await?
        .rows_affected())
    }

    pub async fn exists_room(db_pool: &MySqlPool, stream_id: &str) -> bool {
        query!(r#"SELECT * FROM rooms WHERE stream_id = ?"#, stream_id)
            .fetch_optional(db_pool)
            .await
            .unwrap()
            .is_some()
    }

    pub async fn exists_user(db_pool: &MySqlPool, username: &str, email: &str) -> bool {
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
        stream_id: &str,
        open: bool,
        owner_uuid: &str,
    ) -> Result<u64> {
        Ok(query!(
            r#"UPDATE rooms SET open = ? WHERE stream_id = ? AND owner_uuid = ?"#,
            open,
            stream_id,
            owner_uuid
        )
        .execute(db_pool)
        .await?
        .rows_affected())
    }

    pub async fn assign_stream_token(db_pool: &MySqlPool, stream_id: &str) -> Result<Uuid> {
        let uuid = Uuid::new_v4();
        let uuid_str = uuid.to_simple().to_string();
        query!(
            r#"UPDATE rooms SET stream_token = ? WHERE stream_id = ?"#,
            uuid_str,
            stream_id
        )
        .execute(db_pool)
        .await?;
        Ok(uuid)
    }

    pub async fn unset_stream_token(db_pool: &MySqlPool, stream_id: &str) -> Result<()> {
        query!(
            r#"UPDATE rooms SET stream_token = NULL WHERE stream_id = ?"#,
            stream_id
        )
        .execute(db_pool)
        .await?;
        Ok(())
    }

    pub async fn create_user(
        db_pool: &MySqlPool,
        name: &str,
        email: &str,
        pass: &str,
    ) -> Result<Uuid> {
        let uuid = Uuid::new_v4();
        let uuid_str = uuid.to_simple().to_string();
        let pass_str = hash(pass, 4)?;
        let pass = pass_str.as_bytes();
        let _ = query!(
            r#"INSERT INTO users (uuid, username, email, passwd) VALUES(?, ?, ?, ?)"#,
            uuid_str,
            name,
            email,
            pass
        )
        .execute(db_pool)
        .await?;
        Ok(uuid)
    }

    pub async fn check_password_email(
        db_pool: &MySqlPool,
        email: &str,
        pass: &str,
    ) -> Result<Option<Uuid>> {
        let raw = query_as!(RawUser, r#"SELECT * FROM users WHERE email = ?"#, email)
            .fetch_optional(db_pool)
            .await?;
        if let Some(raw) = raw {
            let real = String::from_utf8(raw.passwd)?;
            if verify(pass, &real)? {
                Ok(Some(Uuid::parse_str(&raw.uuid)?))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub async fn check_password_uuid(
        db_pool: &MySqlPool,
        uuid: &Uuid,
        pass: &str,
    ) -> Result<Option<Uuid>> {
        let uuid_str = uuid.to_simple_ref().to_string();
        let raw = query_as!(RawUser, r#"SELECT * FROM users WHERE uuid = ?"#, uuid_str)
            .fetch_optional(db_pool)
            .await?;
        if let Some(raw) = raw {
            let real = String::from_utf8(raw.passwd)?;
            if verify(pass, &real)? {
                Ok(Some(Uuid::parse_str(&raw.uuid)?))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub async fn update_password(db_pool: &MySqlPool, uuid: &str, new: &str) -> Result<()> {
        let pass_str = hash(new, 4)?;
        let pass = pass_str.as_bytes();
        query!(r#"UPDATE users SET passwd = ? WHERE uuid = ?"#, pass, uuid)
            .execute(db_pool)
            .await?;
        Ok(())
    }
}