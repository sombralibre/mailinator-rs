use dotenv_config::EnvConfig;
use dotenvy::dotenv;

#[derive(Debug, EnvConfig, Clone)]
pub(crate) struct MailinatorConfig {
    #[env_config(
        name = "MAILINATOR_API_URL",
        default = "https://mailinator.com"
    )]
    pub(crate) api_url: String,
    #[env_config(name = "MAILINATOR_API_TOKEN")]
    pub(crate) api_token: Option<String>,
}

impl MailinatorConfig {
    pub fn new() -> Self {
        dotenv().ok();
        MailinatorConfig::init().unwrap()
    }
}
