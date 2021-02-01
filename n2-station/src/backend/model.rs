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
    use rbatis::{crud::CRUD, crud_enable};
    use uuid::Uuid;

    use crate::RBATIS;

    #[crud_enable(table_name:rooms)]
    pub struct RawRoom {
        owner_uuid: String,
        stream_id: String,
        title: String,
        description: String,
        tag: Option<String>,
        open: i8,
        stream_token: Option<String>,
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

    #[crud_enable(table_name:users)]
    pub struct RawUser {
        pub(super) uuid: String,
        username: String,
        email: String,
        pub(super) passwd: String,
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

    impl RawRoom {
        pub fn new(
            owner_uuid: &str,
            stream_id: &str,
            title: &str,
            desc: &str,
            tag: Option<String>,
        ) -> Self {
            Self {
                owner_uuid: String::from(owner_uuid),
                stream_id: String::from(stream_id),
                title: String::from(title),
                description: String::from(desc),
                tag,
                open: 0,
                stream_token: None,
            }
        }

        pub async fn bake(&self, detail: bool) -> Result<BakedRoom> {
            let tag = self
                .tag
                .clone()
                .unwrap_or(String::from(""))
                .split(';')
                .map(|s| s.to_string())
                .collect();
            let status = if self.is_open() { "open" } else { "close" };
            let uuid = Uuid::parse_str(self.owner_uuid.as_str())?;
            let wrapper = RBATIS.new_wrapper().eq("uuid", &self.owner_uuid);
            let raw: RawUser = RBATIS.fetch_by_wrapper("", &wrapper).await?;
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
                desc: self.description.clone(),
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

        pub fn owner_match(&self, uuid: &str) -> bool {
            self.owner_uuid == uuid
        }
    }

    impl RawUser {
        pub fn new(uuid: &str, name: &str, email: &str, passwd: &str) -> Self {
            Self {
                uuid: String::from(uuid),
                username: String::from(name),
                email: String::from(email),
                passwd: String::from(passwd),
            }
        }

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
    use crate::RBATIS;
    use anyhow::Result;
    use bcrypt::{hash, verify};
    use futures::{stream, StreamExt};
    use rbatis::crud::CRUD;
    use serde_json::json;
    use uuid::Uuid;

    pub async fn get_all_rooms() -> Result<Vec<BakedRoom>> {
        Ok(stream::iter(RBATIS.list::<RawRoom>("").await?)
            .then(|raw| async move { raw.bake(false).await.unwrap() })
            .collect()
            .await)
    }

    pub async fn search_rooms_by_owner(owner: &str) -> Result<Vec<BakedRoom>> {
        let wrapper = RBATIS.new_wrapper().eq("uuid", owner);
        Ok(
            stream::iter(RBATIS.list_by_wrapper::<RawRoom>("", &wrapper).await?)
                .then(|raw| async move { raw.bake(false).await.unwrap() })
                .collect()
                .await,
        )
    }

    pub async fn raw_room_by_stream_name(stream_name: &str) -> Result<Option<RawRoom>> {
        let wrapper = RBATIS.new_wrapper().eq("stream_id", stream_name);
        Ok(RBATIS.fetch_by_wrapper("", &wrapper).await?)
    }

    pub async fn raw_rooms_for_user(user: &str) -> Result<u64> {
        Ok(RBATIS
            .exec_prepare(
                "",
                r#"SELECT * FROM rooms WHERE owner_uuid = ?"#,
                &vec![json!(user)],
            )
            .await?
            .rows_affected)
    }

    pub async fn raw_open_rooms() -> Result<u64> {
        Ok(RBATIS
            .exec("", "SELECT * FROM rooms WHERE open = TRUE")
            .await?
            .rows_affected)
    }

    pub async fn search_room_by_stream_name(
        stream_name: &str,
        owner_uuid: &str,
        detail: bool,
    ) -> Result<Option<BakedRoom>> {
        let query = raw_room_by_stream_name(stream_name).await?;
        if let Some(raw) = query {
            if detail && raw.owner_match(owner_uuid) {
                Ok(Some(raw.bake(detail).await?))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub async fn get_all_users() -> Result<Vec<BakedUser>> {
        Ok(RBATIS.list("").await?.iter().map(RawUser::bake).collect())
    }

    pub async fn search_user_by_uuid(uuid: Uuid) -> Result<Option<BakedUser>> {
        let uuid = uuid.to_simple().to_string();
        let wrapper = RBATIS.new_wrapper().eq("uuid", uuid);
        let query: Option<RawUser> = RBATIS.fetch_by_wrapper("", &wrapper).await?;
        if let Some(user) = query {
            Ok(Some(user.bake()))
        } else {
            Ok(None)
        }
    }

    pub async fn search_user_by_name(name: &str) -> Result<Option<BakedUser>> {
        let wrapper = RBATIS.new_wrapper().eq("username", name);
        let query: Option<RawUser> = RBATIS.fetch_by_wrapper("", &wrapper).await?;
        if let Some(user) = query {
            Ok(Some(user.bake()))
        } else {
            Ok(None)
        }
    }

    pub async fn create_room(
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
        RBATIS
            .save("", &RawRoom::new(creator, stream_id, title, desc, tag))
            .await?;
        Ok(())
    }

    pub async fn delete_room(stream_id: &str, owner_uuid: &str) -> Result<u64> {
        let wrapper = RBATIS
            .new_wrapper()
            .eq("stream_id", stream_id)
            .and()
            .eq("owner_uuid", owner_uuid);
        Ok(RBATIS.remove_by_wrapper::<RawRoom>("", &wrapper).await?)
    }

    pub async fn exists_room(stream_id: &str) -> bool {
        RBATIS
            .exec_prepare(
                "",
                "SELECT * FROM rooms WHERE stream_id = ?",
                &vec![json!(stream_id)],
            )
            .await
            .unwrap()
            .rows_affected
            > 0
    }

    pub async fn exists_user(username: &str, email: &str) -> bool {
        RBATIS
            .exec_prepare(
                "",
                "SELECT * FROM users WHERE username = ? OR email = ?",
                &vec![json!(username), json!(email)],
            )
            .await
            .unwrap()
            .rows_affected
            > 0
    }

    pub async fn set_room_status(stream_id: &str, open: bool, owner_uuid: &str) -> Result<u64> {
        Ok(RBATIS
            .exec_prepare(
                "",
                r#"UPDATE rooms SET open = ? WHERE stream_id = ? AND owner_uuid = ?"#,
                &vec![json!(open), json!(stream_id), json!(owner_uuid)],
            )
            .await?
            .rows_affected)
    }

    pub async fn assign_stream_token(stream_id: &str) -> Result<Uuid> {
        let uuid = Uuid::new_v4();
        let uuid_str = uuid.to_simple().to_string();
        RBATIS
            .exec_prepare(
                "",
                r#"UPDATE rooms SET stream_token = ? WHERE stream_id = ?"#,
                &vec![json!(uuid_str), json!(stream_id)],
            )
            .await?;
        Ok(uuid)
    }

    pub async fn unset_stream_token(stream_id: &str) -> Result<()> {
        RBATIS
            .exec_prepare(
                "",
                r#"UPDATE rooms SET stream_token = NULL WHERE stream_id = ?"#,
                &vec![json!(stream_id)],
            )
            .await?;
        Ok(())
    }

    pub async fn create_user(name: &str, email: &str, pass: &str) -> Result<Uuid> {
        let uuid = Uuid::new_v4();
        let uuid_str = uuid.to_simple().to_string();
        let pass = hash(pass, 4)?;
        RBATIS
            .save("", &RawUser::new(&uuid_str, name, email, &pass))
            .await?;
        Ok(uuid)
    }

    pub async fn check_password_email(email: &str, pass: &str) -> Result<Option<Uuid>> {
        let wrapper = RBATIS.new_wrapper().eq("email", email);
        let raw: Option<RawUser> = RBATIS.fetch_by_wrapper("", &wrapper).await?;
        if let Some(raw) = raw {
            if verify(pass, &raw.passwd)? {
                Ok(Some(Uuid::parse_str(&raw.uuid)?))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub async fn check_password_uuid(uuid: &Uuid, pass: &str) -> Result<Option<Uuid>> {
        let uuid_str = uuid.to_simple_ref().to_string();
        let wrapper = RBATIS.new_wrapper().eq("uuid", uuid_str);
        let raw: Option<RawUser> = RBATIS.fetch_by_wrapper("", &wrapper).await?;
        if let Some(raw) = raw {
            if verify(pass, &raw.passwd)? {
                Ok(Some(Uuid::parse_str(&raw.uuid)?))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub async fn update_password(uuid: &str, new: &str) -> Result<()> {
        let pass_str = hash(new, 4)?;
        let pass = pass_str.as_bytes();
        RBATIS
            .exec_prepare(
                "",
                r#"UPDATE users SET passwd = ? WHERE uuid = ?"#,
                &vec![json!(pass), json!(uuid)],
            )
            .await?;
        Ok(())
    }
}
