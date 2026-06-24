mod bartec;
mod collection;
mod config;
mod error;
mod notify;

use std::process;

use bartec::BartecClient;
use collection::CollectionSource;
use config::Config;
use error::Error;
use notify::{Notifier, NtfyNotifier};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    Morning,
    Evening,
}

fn parse_mode_from(arg: Option<&str>) -> Result<Mode, String> {
    match arg {
        Some("morning") => Ok(Mode::Morning),
        Some("evening") => Ok(Mode::Evening),
        Some(other) => Err(format!(
            "Unknown mode '{other}'. Expected 'morning' or 'evening'."
        )),
        None => Err("No mode specified. Expected 'morning' or 'evening'.".to_string()),
    }
}

fn parse_mode() -> Result<Mode, String> {
    parse_mode_from(std::env::args().nth(1).as_deref())
}

fn normalise_url(url: &str) -> String {
    url.trim_end_matches('/').to_string()
}

fn run<S, N>(mode: Mode, source: &S, notifier: &N) -> Result<(), Error>
where
    S: CollectionSource,
    N: Notifier,
{
    todo!()
}

fn main() {
    // ntfy config is read before everything else so every failure path can notify.
    let ntfy_topic = std::env::var("BINMAN_NTFY_TOPIC").ok();
    let ntfy_url = std::env::var("BINMAN_NTFY_URL")
        .map(|u| normalise_url(&u))
        .unwrap_or_else(|_| "https://ntfy.sh".to_string());

    let try_notify = |message: &str| {
        if let Some(ref topic) = ntfy_topic {
            let notifier = NtfyNotifier::new(topic.as_str(), ntfy_url.as_str());
            if let Err(e) = notifier.send("binman error", message) {
                eprintln!("Warning: failed to send ntfy notification: {e}");
            }
        }
    };

    let mode = match parse_mode() {
        Ok(m) => m,
        Err(e) => {
            try_notify(&e);
            eprintln!("Error: {e}");
            process::exit(1);
        }
    };

    let config = match Config::from_env() {
        Ok(c) => c,
        Err(e) => {
            try_notify(&e.to_string());
            eprintln!("Error: {e}");
            process::exit(1);
        }
    };

    let source = BartecClient::new(&config.uprn);
    let notifier = NtfyNotifier::new(&config.ntfy_topic, &config.ntfy_url);

    if let Err(e) = run(mode, &source, &notifier) {
        try_notify(&e.to_string());
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mode_morning() {
        assert_eq!(parse_mode_from(Some("morning")), Ok(Mode::Morning));
    }

    #[test]
    fn parse_mode_evening() {
        assert_eq!(parse_mode_from(Some("evening")), Ok(Mode::Evening));
    }

    #[test]
    fn parse_mode_unknown_arg() {
        assert!(parse_mode_from(Some("noon")).is_err());
    }

    #[test]
    fn parse_mode_no_arg() {
        assert!(parse_mode_from(None).is_err());
    }

    #[test]
    fn normalise_url_strips_trailing_slash() {
        assert_eq!(normalise_url("https://ntfy.sh/"), "https://ntfy.sh");
    }

    #[test]
    fn normalise_url_no_change_when_clean() {
        assert_eq!(normalise_url("https://ntfy.sh"), "https://ntfy.sh");
    }

    #[test]
    fn normalise_url_strips_multiple_trailing_slashes() {
        assert_eq!(normalise_url("https://ntfy.sh///"), "https://ntfy.sh");
    }
}
