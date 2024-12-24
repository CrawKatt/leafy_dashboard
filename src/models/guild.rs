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

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordChannel {
    pub id: String,
    pub r#type: u8, // El tipo de canal (0 = texto, 2 = voz, etc.)
    pub guild_id: Option<String>,
    pub name: String,
    pub parent_id: Option<String>,
    pub position: i32,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub nsfw: Option<bool>,
    pub topic: Option<String>, // Solo para canales de texto
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionOverwrite {
    pub id: String,
    pub r#type: u8, // 0 = rol, 1 = usuario
    pub allow: String,
    pub deny: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordUser {
    pub name: String,
    pub id: String,
    pub r#type: u8, // 0 = rol, 1 = usuario
    pub allow: String,
    pub deny: String,
}