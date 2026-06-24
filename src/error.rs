#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Missing required environment variable: {0}")]
    MissingEnvVar(&'static str),
    #[error("Collection API request failed: {0}")]
    ApiRequest(String),
    #[error("Collection API returned unexpected data: {0}")]
    ApiParse(String),
    #[error("Failed to send ntfy notification: {0}")]
    Notify(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_env_var_message() {
        assert_eq!(
            Error::MissingEnvVar("BINMAN_UPRN").to_string(),
            "Missing required environment variable: BINMAN_UPRN"
        );
    }

    #[test]
    fn api_request_message() {
        assert_eq!(
            Error::ApiRequest("connection refused".to_string()).to_string(),
            "Collection API request failed: connection refused"
        );
    }

    #[test]
    fn api_parse_message() {
        assert_eq!(
            Error::ApiParse("unexpected field".to_string()).to_string(),
            "Collection API returned unexpected data: unexpected field"
        );
    }

    #[test]
    fn notify_message() {
        assert_eq!(
            Error::Notify("timeout".to_string()).to_string(),
            "Failed to send ntfy notification: timeout"
        );
    }
}
