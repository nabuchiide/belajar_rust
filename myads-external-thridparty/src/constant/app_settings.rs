use envconfig::Envconfig;

#[derive(Envconfig, Debug)]
pub struct AppSettings {
    #[envconfig(from = "SERVER_PORT", default = "8090")]
    server_port: u16,
    #[envconfig(from = "DATABASE_URL")]
    database_url: String,

}

impl AppSettings {
    pub fn server_port(&self) -> u16 {
        self.server_port
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }
}