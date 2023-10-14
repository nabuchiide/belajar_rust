use envconfig::Envconfig;

#[derive(Envconfig, Debug)]
pub struct AppSettings {
    #[envconfig(from = "SERVER_PORT", default = "8090")]
    server_port: u16,
    #[envconfig(from = "DATABASE_URL")]
    database_url: String,
    #[envconfig(from = "REDIS_CLUSTER_NODE")]
    redis_cluster_node: String,
}

impl AppSettings {
    pub fn server_port(&self) -> u16 {
        self.server_port
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn redis_cluster_node(&self) -> &str {
        &self.redis_cluster_node
    }

}