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
    let tomorrow = chrono::Local::now().date_naive() + chrono::Duration::days(1);
    let collections = source.fetch_upcoming()?;

    let Some(collection) = collections.iter().find(|c| c.date == tomorrow) else {
        return Ok(());
    };

    let bin_list = collection
        .bins
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let (title, body) = match mode {
        Mode::Morning => ("Bin day today", format!("Putting out: {bin_list}")),
        Mode::Evening => ("Put your bins out!", format!("Tonight: {bin_list}")),
    };

    notifier.send(title, &body)
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
    use std::cell::RefCell;

    use chrono::NaiveDate;

    use collection::{BinType, Collection};

    use super::*;

    // ── mock helpers ────────────────────────────────────────────────────────

    struct MockSource(Vec<Collection>);

    impl CollectionSource for MockSource {
        fn fetch_upcoming(&self) -> Result<Vec<Collection>, Error> {
            Ok(self.0.clone())
        }
    }

    struct FailingSource;

    impl CollectionSource for FailingSource {
        fn fetch_upcoming(&self) -> Result<Vec<Collection>, Error> {
            Err(Error::ApiRequest("mock error".to_string()))
        }
    }

    struct RecordingNotifier(RefCell<Vec<(String, String)>>);

    impl RecordingNotifier {
        fn new() -> Self {
            Self(RefCell::new(vec![]))
        }
        fn calls(&self) -> Vec<(String, String)> {
            self.0.borrow().clone()
        }
    }

    impl Notifier for RecordingNotifier {
        fn send(&self, title: &str, message: &str) -> Result<(), Error> {
            self.0.borrow_mut().push((title.to_string(), message.to_string()));
            Ok(())
        }
    }

    fn today() -> NaiveDate {
        chrono::Local::now().date_naive()
    }

    fn collection_on(date: NaiveDate, bins: Vec<BinType>) -> Collection {
        Collection { date, bins }
    }

    // ── run() tests ─────────────────────────────────────────────────────────

    #[test]
    fn run_silent_when_no_collection_today() {
        let source = MockSource(vec![]);
        let notifier = RecordingNotifier::new();
        run(Mode::Morning, &source, &notifier).unwrap();
        assert!(notifier.calls().is_empty());
    }

    fn tomorrow() -> NaiveDate {
        today() + chrono::Duration::days(1)
    }

    #[test]
    fn run_silent_when_no_collection_tomorrow() {
        // collection is today, not tomorrow — both modes should be silent
        let source = MockSource(vec![collection_on(today(), vec![BinType::GeneralWaste])]);
        for mode in [Mode::Morning, Mode::Evening] {
            let notifier = RecordingNotifier::new();
            run(mode, &source, &notifier).unwrap();
            assert!(notifier.calls().is_empty(), "{mode:?} should be silent");
        }
    }

    #[test]
    fn run_morning_sends_notification_when_collection_is_tomorrow() {
        let source = MockSource(vec![collection_on(tomorrow(), vec![BinType::Recycling])]);
        let notifier = RecordingNotifier::new();
        run(Mode::Morning, &source, &notifier).unwrap();
        let calls = notifier.calls();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, "Bin day today");
        assert!(calls[0].1.contains("recycling"));
    }

    #[test]
    fn run_evening_sends_notification_when_collection_is_tomorrow() {
        let source = MockSource(vec![collection_on(tomorrow(), vec![BinType::GeneralWaste])]);
        let notifier = RecordingNotifier::new();
        run(Mode::Evening, &source, &notifier).unwrap();
        let calls = notifier.calls();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, "Put your bins out!");
        assert!(calls[0].1.contains("general waste"));
    }

    #[test]
    fn run_lists_multiple_bins() {
        let source = MockSource(vec![collection_on(
            tomorrow(),
            vec![BinType::Recycling, BinType::GardenWaste],
        )]);
        let notifier = RecordingNotifier::new();
        run(Mode::Morning, &source, &notifier).unwrap();
        let body = &notifier.calls()[0].1;
        assert!(body.contains("recycling"));
        assert!(body.contains("garden waste"));
    }

    #[test]
    fn run_propagates_source_error() {
        let notifier = RecordingNotifier::new();
        assert!(run(Mode::Morning, &FailingSource, &notifier).is_err());
    }

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
