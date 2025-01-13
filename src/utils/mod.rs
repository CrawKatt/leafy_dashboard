/// Uso de `env!` requiere de la definición de la variable desde comandos
/// `export BASE_URL=<url_aquí>`
/// SI NO SE DEFINE PREVIAMENTE HABRÁ ERROR DE COMPILACIÓN
pub const BASE_URL: &str = env!("BASE_URL");