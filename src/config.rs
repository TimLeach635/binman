use crate::error::Error;

pub struct Config {
    pub uprn: String,
    pub ntfy_topic: String,
    pub ntfy_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {
        Self::from_env_with(|key| std::env::var(key).ok())
    }

    fn from_env_with(get: impl Fn(&str) -> Option<String>) -> Result<Self, Error> {
        let uprn = get("BINMAN_UPRN").ok_or(Error::MissingEnvVar("BINMAN_UPRN"))?;
        let ntfy_topic =
            get("BINMAN_NTFY_TOPIC").ok_or(Error::MissingEnvVar("BINMAN_NTFY_TOPIC"))?;
        let ntfy_url = get("BINMAN_NTFY_URL")
            .unwrap_or_else(|| "https://ntfy.sh".to_string());
        let ntfy_url = ntfy_url.trim_end_matches('/').to_string();

        Ok(Self { uprn, ntfy_topic, ntfy_url })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env(uprn: Option<&str>, topic: Option<&str>, url: Option<&str>) -> impl Fn(&str) -> Option<String> {
        let uprn = uprn.map(str::to_string);
        let topic = topic.map(str::to_string);
        let url = url.map(str::to_string);
        move |key| match key {
            "BINMAN_UPRN" => uprn.clone(),
            "BINMAN_NTFY_TOPIC" => topic.clone(),
            "BINMAN_NTFY_URL" => url.clone(),
            _ => None,
        }
    }

    #[test]
    fn missing_uprn_returns_error() {
        let result = Config::from_env_with(env(None, Some("test-topic"), None));
        assert!(matches!(result, Err(Error::MissingEnvVar("BINMAN_UPRN"))));
    }

    #[test]
    fn missing_ntfy_topic_returns_error() {
        let result = Config::from_env_with(env(Some("100050403003"), None, None));
        assert!(matches!(result, Err(Error::MissingEnvVar("BINMAN_NTFY_TOPIC"))));
    }

    #[test]
    fn ntfy_url_defaults_to_ntfy_sh() {
        let config = Config::from_env_with(env(Some("100050403003"), Some("test-topic"), None))
            .unwrap();
        assert_eq!(config.ntfy_url, "https://ntfy.sh");
    }

    #[test]
    fn ntfy_url_trailing_slash_is_stripped() {
        let config = Config::from_env_with(env(
            Some("100050403003"),
            Some("test-topic"),
            Some("https://my-ntfy.example.com/"),
        ))
        .unwrap();
        assert_eq!(config.ntfy_url, "https://my-ntfy.example.com");
    }

    #[test]
    fn all_fields_set_correctly() {
        let config = Config::from_env_with(env(
            Some("100050403003"),
            Some("my-bins"),
            Some("https://ntfy.example.com"),
        ))
        .unwrap();
        assert_eq!(config.uprn, "100050403003");
        assert_eq!(config.ntfy_topic, "my-bins");
        assert_eq!(config.ntfy_url, "https://ntfy.example.com");
    }
}
