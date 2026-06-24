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
        todo!()
    }
}

impl Notifier for NtfyNotifier {
    fn send(&self, title: &str, message: &str) -> Result<(), Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "requires NtfyNotifier::new() and send() implementation"]
    fn send_delivers_to_correct_endpoint() {
        // Verify POST goes to {base_url}/{topic} with the right title and body.
        // let notifier = NtfyNotifier::new("test-topic", "https://ntfy.sh");
        // notifier.send("test title", "test message").unwrap();
    }

    #[test]
    #[ignore = "requires NtfyNotifier::new() and send() implementation"]
    fn send_returns_err_on_network_failure() {
        // let notifier = NtfyNotifier::new("test-topic", "http://localhost:9999");
        // assert!(notifier.send("title", "message").is_err());
    }
}
