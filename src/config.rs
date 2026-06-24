use crate::error::Error;

pub struct Config {
    /// UPRN must consist only of digits — validate with `str::chars().all(|c| c.is_ascii_digit())`.
    pub uprn: String,
    pub ntfy_topic: String,
    /// Must have trailing slashes stripped — use `str::trim_end_matches('/')`.
    /// Defaults to `"https://ntfy.sh"` when `BINMAN_NTFY_URL` is unset.
    pub ntfy_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "requires Config::from_env() implementation"]
    fn missing_uprn_returns_error() {
        // std::env::remove_var("BINMAN_UPRN");
        // std::env::set_var("BINMAN_NTFY_TOPIC", "test-topic");
        // assert!(matches!(
        //     Config::from_env(),
        //     Err(Error::MissingEnvVar("BINMAN_UPRN"))
        // ));
    }

    #[test]
    #[ignore = "requires Config::from_env() implementation"]
    fn missing_ntfy_topic_returns_error() {
        // std::env::set_var("BINMAN_UPRN", "100050403003");
        // std::env::remove_var("BINMAN_NTFY_TOPIC");
        // assert!(matches!(
        //     Config::from_env(),
        //     Err(Error::MissingEnvVar("BINMAN_NTFY_TOPIC"))
        // ));
    }

    #[test]
    #[ignore = "requires Config::from_env() implementation"]
    fn ntfy_url_defaults_to_ntfy_sh() {
        // std::env::set_var("BINMAN_UPRN", "100050403003");
        // std::env::set_var("BINMAN_NTFY_TOPIC", "test-topic");
        // std::env::remove_var("BINMAN_NTFY_URL");
        // let config = Config::from_env().unwrap();
        // assert_eq!(config.ntfy_url, "https://ntfy.sh");
    }

    #[test]
    #[ignore = "requires Config::from_env() implementation"]
    fn ntfy_url_trailing_slash_is_stripped() {
        // std::env::set_var("BINMAN_UPRN", "100050403003");
        // std::env::set_var("BINMAN_NTFY_TOPIC", "test-topic");
        // std::env::set_var("BINMAN_NTFY_URL", "https://my-ntfy.example.com/");
        // let config = Config::from_env().unwrap();
        // assert_eq!(config.ntfy_url, "https://my-ntfy.example.com");
    }
}
