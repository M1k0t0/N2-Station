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
        pub tag: String,
        pub icon: String,
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
    use rbatis::{crud::CRUD, crud_table};
    use uuid::Uuid;

    use crate::RBATIS;

    #[crud_table(table_name:"rooms"|id_name:"stream_id")]
    pub struct RawRoom {
        pub(crate) owner_uuid: String,
        pub(crate) stream_id: String,
        pub(crate) title: String,
        pub(crate) description: String,
        pub(crate) tag: Option<String>,
        pub(crate) open: i8,
        pub(crate) stream_token: Option<String>,
        pub(crate) room_icon: String,
    }

    #[derive(serde::Serialize, Clone, Default)]
    pub struct BakedRoom {
        pub(crate) stream_id: String,
        pub(crate) title: String,
        pub(crate) desc: String,
        pub(crate) tag: Vec<String>,
        pub(crate) status: String,
        pub(crate) user: BakedUser,
        pub(crate) stream_token: Option<Uuid>,
        pub(crate) room_icon: String,
    }

    #[crud_table(table_name:"users"|id_name:"uuid")]
    pub struct RawUser {
        pub(super) uuid: String,
        username: String,
        email: String,
        pub(super) passwd: String,
    }

    #[derive(serde::Serialize, Clone, Default)]
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
            room_icon: &str,
        ) -> Self {
            Self {
                owner_uuid: String::from(owner_uuid),
                stream_id: String::from(stream_id),
                title: String::from(title),
                description: String::from(desc),
                tag,
                open: 0,
                stream_token: None,
                room_icon: String::from(room_icon),
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
            let wrapper = RBATIS.new_wrapper().eq("uuid", &self.owner_uuid);
            let raw: RawUser = RBATIS.fetch_by_wrapper(wrapper).await?;
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
                stream_id: self.stream_id.clone(),
                title: self.title.clone(),
                desc: self.description.clone(),
                tag: tag,
                status: status.to_string(),
                user: raw.bake(),
                stream_token,
                room_icon: self.room_icon.clone(),
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

    impl BakedUser {
        pub fn name(&self) -> String {
            self.name.clone()
        }
    }
}

pub mod handler {
    use super::response::{BakedRoom, BakedUser, RawRoom, RawUser};
    use crate::{
        backend::{incremental::IncrementalData, IncrementalServer},
        RBATIS,
    };
    use actix::Addr;
    use anyhow::Result;
    use bcrypt::{hash, verify};
    use futures::{stream, StreamExt};
    use rbatis::{crud::CRUD, executor::Executor};
    use uuid::Uuid;

    pub async fn get_all_rooms() -> Result<Vec<BakedRoom>> {
        Ok(stream::iter(RBATIS.fetch_list::<RawRoom>().await?)
            .then(|raw| async move { raw.bake(false).await.unwrap() })
            .collect()
            .await)
    }

    pub async fn search_rooms_by_owner(owner: &str) -> Result<Vec<BakedRoom>> {
        let wrapper = RBATIS.new_wrapper().eq("owner_uuid", owner);
        Ok(
            stream::iter(RBATIS.fetch_list_by_wrapper::<RawRoom>(wrapper).await?)
                .then(|raw| async move { raw.bake(false).await.unwrap() })
                .collect()
                .await,
        )
    }

    pub async fn raw_room_by_stream_name(stream_name: &str) -> Result<Option<RawRoom>> {
        let wrapper = RBATIS.new_wrapper().eq("stream_id", stream_name);
        Ok(RBATIS.fetch_by_wrapper(wrapper).await?)
    }

    pub async fn raw_rooms_for_user(user: &str) -> Result<u64> {
        Ok(RBATIS
            .exec(
                r#"SELECT * FROM rooms WHERE owner_uuid = ?"#,
                vec![rbson::bson!(user)],
            )
            .await?
            .rows_affected)
    }

    pub async fn raw_open_rooms() -> Result<u64> {
        Ok(RBATIS
            .exec("SELECT * FROM rooms WHERE open = TRUE", vec![])
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
            if detail && !raw.owner_match(owner_uuid) {
                Ok(None)
            } else {
                Ok(Some(raw.bake(detail).await?))
            }
        } else {
            Ok(None)
        }
    }

    pub async fn get_all_users() -> Result<Vec<BakedUser>> {
        Ok(RBATIS
            .fetch_list()
            .await?
            .iter()
            .map(RawUser::bake)
            .collect())
    }

    pub async fn search_user_by_uuid(uuid: Uuid) -> Result<Option<BakedUser>> {
        let uuid = uuid.to_simple().to_string();
        let wrapper = RBATIS.new_wrapper().eq("uuid", uuid);
        let query: Option<RawUser> = RBATIS.fetch_by_wrapper(wrapper).await?;
        if let Some(user) = query {
            Ok(Some(user.bake()))
        } else {
            Ok(None)
        }
    }

    pub async fn search_user_by_name(name: &str) -> Result<Option<BakedUser>> {
        let wrapper = RBATIS.new_wrapper().eq("username", name);
        let query: Option<RawUser> = RBATIS.fetch_by_wrapper(wrapper).await?;
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
        tag: &str,
        icon: &str,
        svr: &Addr<IncrementalServer>,
    ) -> Result<()> {
        let tag = if tag.is_empty() {
            None
        } else {
            let tag: Vec<String> = serde_json::from_str(tag)?;
            if tag.len() > 0 && tag.len() <= 20 {
                Some(tag.join(";"))
            } else {
                None
            }
        };
        let raw = RawRoom::new(creator, stream_id, title, desc, tag, icon);
        RBATIS.save(&raw, &[]).await?;
        let _ = svr.do_send(IncrementalData {
            data: raw.bake(false).await.unwrap(),
        });
        Ok(())
    }

    pub async fn delete_room(
        stream_id: &str,
        owner_uuid: &str,
        svr: &Addr<IncrementalServer>,
    ) -> Result<u64> {
        let wrapper = RBATIS
            .new_wrapper()
            .eq("stream_id", stream_id)
            .and()
            .eq("owner_uuid", owner_uuid);
        let _ = svr.do_send(IncrementalData {
            data: BakedRoom {
                stream_id: stream_id.to_owned(),
                ..Default::default()
            },
        });
        Ok(RBATIS.remove_by_wrapper::<RawRoom>(wrapper).await?)
    }

    pub async fn exists_room(stream_id: &str) -> bool {
        RBATIS
            .exec(
                "SELECT * FROM rooms WHERE stream_id = ?",
                vec![rbson::bson!(stream_id)],
            )
            .await
            .unwrap()
            .rows_affected
            > 0
    }

    pub async fn exists_user(username: &str, email: &str) -> bool {
        RBATIS
            .exec(
                "SELECT * FROM users WHERE username = ? OR email = ?",
                vec![rbson::bson!(username), rbson::bson!(email)],
            )
            .await
            .unwrap()
            .rows_affected
            > 0
    }

    pub async fn set_room_status(
        stream_id: &str,
        open: bool,
        owner_uuid: &str,
        svr: &Addr<IncrementalServer>,
    ) -> Result<u64> {
        let wrapper = RBATIS.new_wrapper().eq("stream_id", stream_id);
        let raw = RBATIS.fetch_by_wrapper::<RawRoom>(wrapper).await.unwrap();
        let raw = RawRoom {
            open: if open { 1 } else { 0 },
            ..raw
        };
        let _ = svr.do_send(IncrementalData {
            data: raw.bake(false).await.unwrap(),
        });
        Ok(RBATIS
            .exec(
                r#"UPDATE rooms SET open = ? WHERE stream_id = ? AND owner_uuid = ?"#,
                vec![
                    rbson::bson!(open),
                    rbson::bson!(stream_id),
                    rbson::bson!(owner_uuid),
                ],
            )
            .await?
            .rows_affected)
    }

    pub async fn assign_stream_token(stream_id: &str) -> Result<Uuid> {
        let uuid = Uuid::new_v4();
        let uuid_str = uuid.to_simple().to_string();
        RBATIS
            .exec(
                r#"UPDATE rooms SET stream_token = ? WHERE stream_id = ?"#,
                vec![rbson::bson!(uuid_str), rbson::bson!(stream_id)],
            )
            .await?;
        Ok(uuid)
    }

    pub async fn unset_stream_token(stream_id: &str) -> Result<()> {
        RBATIS
            .exec(
                r#"UPDATE rooms SET stream_token = NULL WHERE stream_id = ?"#,
                vec![rbson::bson!(stream_id)],
            )
            .await?;
        Ok(())
    }

    pub async fn create_user(name: &str, email: &str, pass: &str) -> Result<Uuid> {
        let uuid = Uuid::new_v4();
        let uuid_str = uuid.to_simple().to_string();
        let pass = hash(pass, 4)?;
        RBATIS
            .save(&RawUser::new(&uuid_str, name, email, &pass), &[])
            .await?;
        Ok(uuid)
    }

    pub async fn check_password_email(email: &str, pass: &str) -> Result<Option<Uuid>> {
        let wrapper = RBATIS.new_wrapper().eq("email", email);
        let raw: Option<RawUser> = RBATIS.fetch_by_wrapper(wrapper).await?;
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
        let raw: Option<RawUser> = RBATIS.fetch_by_wrapper(wrapper).await?;
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
        RBATIS
            .exec(
                r#"UPDATE users SET passwd = ? WHERE uuid = ?"#,
                vec![rbson::bson!(pass_str), rbson::bson!(uuid)],
            )
            .await?;
        Ok(())
    }
}
