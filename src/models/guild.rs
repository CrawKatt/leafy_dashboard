use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct DiscordServer {
    pub guild_id: String,
    pub name: String,
    pub owner: String,
    pub icon: Option<String>,
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