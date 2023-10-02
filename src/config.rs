use std::sync::OnceLock;

pub struct Config {
    pub discord_token: String,
    pub channel_id_to_post_reminders: u64,
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

        let channel_id_to_post_reminders = dotenvy::var("CHANNEL_ID_TO_POST_REMINDERS")
            .expect("No channel ID found in environment. Please set CHANNEL_ID_TO_POST_REMINDERS.")
            .parse::<u64>()
            .expect("CHANNEL_ID_TO_POST_REMINDERS is not an integer. Please set CHANNEL_ID_TO_POST_REMINDERS to an integer.");

        Self {
            discord_token,
            channel_id_to_post_reminders,
        }
    }
}
