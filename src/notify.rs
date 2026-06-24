use crate::error::Error;

pub trait Notifier {
    fn send(&self, title: &str, message: &str) -> Result<(), Error>;
}

pub struct NtfyNotifier {
    topic: String,
    base_url: String,
    http: reqwest::blocking::Client,
}

impl NtfyNotifier {
    /// `base_url` must not have a trailing slash — the ntfy endpoint is constructed as
    /// `POST {base_url}/{topic}`, so a trailing slash would produce a double-slash URL.
    pub fn new(topic: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            topic: topic.into(),
            base_url: base_url.into(),
            http: reqwest::blocking::Client::new(),
        }
    }
}

impl Notifier for NtfyNotifier {
    fn send(&self, title: &str, message: &str) -> Result<(), Error> {
        let url = format!("{}/{}", self.base_url, self.topic);
        self.http
            .post(&url)
            .header("Title", title)
            .body(message.to_string())
            .send()
            .map_err(|e| Error::Notify(e.to_string()))?
            .error_for_status()
            .map_err(|e| Error::Notify(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_returns_err_on_network_failure() {
        let notifier = NtfyNotifier::new("test-topic", "http://localhost:19999");
        assert!(notifier.send("title", "message").is_err());
    }

    #[test]
    #[ignore = "integration test: sends a real ntfy notification"]
    fn send_delivers_to_correct_endpoint() {
        let notifier = NtfyNotifier::new("binman-test", "https://ntfy.sh");
        notifier.send("test title", "test message").unwrap();
    }
}
