use std::sync::OnceLock;

pub struct Config {
    pub discord_token: String,
}

pub fn global() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();

    CONFIG.get_or_init(|| Config::from_env())
}

impl Config {
    fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let discord_token = dotenvy::var("DISCORD_TOKEN")
            .expect("No token found in environment. Please set DISCORD_TOKEN.");

        Self { discord_token }
    }
}
