use rust_dotenv::dotenv::DotEnv;

pub struct AppConfig {
    pub smtp_host: String,
    pub smtp_user: String,
    pub smtp_pass: String,
}

fn get_env_var(key: &str, dotenv_obj: &DotEnv) -> String {
    match dotenv_obj.get_var(key.to_string()) {
        Some(value) => value,
        None => panic!("Environment variable {} not found", key),
    }
}

pub fn load_config(dotenv_obj: &DotEnv) -> AppConfig {
    AppConfig {
        smtp_host: get_env_var("SMTP_HOST", dotenv_obj),
        smtp_user: get_env_var("SMTP_USER", dotenv_obj),
        smtp_pass: get_env_var("SMTP_PASS", dotenv_obj),
    }
}
