use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct DiscordServer {
    pub guild_id: String,
    pub name: String,
    pub owner: String,
    pub icon: Option<String>,
}

impl DiscordServer {
    pub const fn new(guild_id: String, name: String, owner: String, icon: Option<String>) -> Self {
        Self {
            guild_id,
            name,
            owner,
            icon
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct DiscordRole {
    pub id: String,
    pub name: String,
    pub color: u32,
    pub hoist: bool,
    pub position: i32,
    pub permissions: String,
    pub managed: bool,
    pub mentionable: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct DiscordChannel {
    pub id: String,
    pub r#type: u8, // El tipo de canal (0 = texto, 2 = voz, etc.)
    pub guild_id: Option<String>,
    pub name: String,
    pub parent_id: Option<String>,
    pub position: i32,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub nsfw: Option<bool>,
    pub topic: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct PermissionOverwrite {
    pub id: String,
    pub r#type: u8, // 0 = rol, 1 = usuario
    pub allow: String,
    pub deny: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct DiscordUser {
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub communication_disabled_until: Option<String>,
    pub flags: u32,
    pub joined_at: String,
    pub nick: Option<String>,
    pub pending: bool,
    pub premium_since: Option<String>,
    pub roles: Vec<String>,
    pub unusual_dm_activity_until: Option<String>,
    pub user: User,
    pub mute: bool,
    pub deaf: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub username: String,
    pub avatar: String,
    pub discriminator: String,
    pub public_flags: u32,
    pub flags: u32,
    pub bot: Option<bool>,
    pub banner: Option<String>,
    pub accent_color: Option<String>,
    pub global_name: Option<String>,
    pub avatar_decoration_data: Option<AvatarDecorationData>,
    pub banner_color: Option<String>,
    pub clan: Option<String>,
    pub primary_guild: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct AvatarDecorationData {
    pub asset: String,
    pub expires_at: Option<String>,
    pub sku_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildData {
    pub admins: Admin,
    pub guild_id: String,
    pub id: Option<RecordId>,
    pub forbidden: Forbidden,
    pub time_out: TimeOut,
    pub channels: Channels,
    pub messages: Messages,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Admin {
    pub role: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Forbidden {
    pub user: String,
    pub role: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeOut {
    pub time: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channels {
    pub welcome: String,
    pub ooc: String,
    pub logs: String,
    pub exceptions: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Messages {
    pub welcome: String,
    pub time_out: String,
    pub warn: String
}

#[allow(dead_code)]
pub trait Getter {
    fn to_guild_id(&self) -> String;
}

impl Getter for RecordId {
    fn to_guild_id(&self) -> String {
        self.key()
            .to_string()
            .trim_matches(|c| c == '⟨' || c == '⟩')
            .to_string()
    }
}