use std::sync::Arc;

use logforth::record::{Level, Record};
use logforth::{Append, Diagnostic, Error};
use rerun::{RecordingStream, StoreKind, TextLogLevel};

fn to_rerun_log_level(level: Level) -> TextLogLevel {
    if level >= Level::Fatal {
        TextLogLevel::CRITICAL.into()
    } else if level >= Level::Error {
        TextLogLevel::ERROR.into()
    } else if level >= Level::Warn {
        TextLogLevel::WARN.into()
    } else if level >= Level::Info {
        TextLogLevel::INFO.into()
    } else if level >= Level::Debug {
        TextLogLevel::DEBUG.into()
    } else {
        TextLogLevel::TRACE.into()
    }
}

/// Where the appender finds the `RecordingStream` to log to.
enum Source {
    /// A stream supplied up front.
    Fixed(RecordingStream),
    /// Rerun's active (global) recording, resolved per record. Records logged while no
    /// recording is active are dropped.
    Active,
}

/// A clock returning seconds, used to stamp records on a Rerun timeline.
pub type Clock = Arc<dyn Fn() -> f64 + Send + Sync>;

/// A [`logforth`] appender that sends every log record to Rerun as a `TextLog`.
pub struct RerunAppender {
    source: Source,
    path: String,
    timeline: Option<(String, Clock)>,
}

impl RerunAppender {
    /// Log to `rec`, under Rerun entity path `path` (e.g. `"logs"`).
    pub fn new(rec: RecordingStream, path: impl Into<String>) -> Self {
        Self {
            source: Source::Fixed(rec),
            path: path.into(),
            timeline: None,
        }
    }

    /// Log to Rerun's active (global) recording, resolved per record. Set the active recording
    /// with `RecordingStream::set_global`; records logged while none is active are dropped.
    pub fn active(path: impl Into<String>) -> Self {
        Self {
            source: Source::Active,
            path: path.into(),
            timeline: None,
        }
    }

    /// Stamp each record on `timeline` at `clock()` seconds, so records align with other data on
    /// that timeline instead of Rerun's default `log_time`.
    pub fn with_timeline(mut self, timeline: impl Into<String>, clock: Clock) -> Self {
        self.timeline = Some((timeline.into(), clock));
        self
    }

    fn stream(&self) -> Option<RecordingStream> {
        match &self.source {
            Source::Fixed(rec) => Some(rec.clone()),
            Source::Active => RecordingStream::global(StoreKind::Recording),
        }
    }
}

impl std::fmt::Debug for RerunAppender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RerunAppender")
            .field("path", &self.path)
            .field("timeline", &self.timeline.as_ref().map(|(t, _)| t))
            .finish_non_exhaustive()
    }
}

impl Append for RerunAppender {
    fn append(&self, record: &Record, _diags: &[Box<dyn Diagnostic>]) -> Result<(), Error> {
        let Some(rec) = self.stream() else {
            return Ok(());
        };
        if let Some((timeline, clock)) = &self.timeline {
            rec.set_duration_secs(timeline.as_str(), clock());
        }
        let msg = format!("{}: {}", record.target(), record.payload());
        let text =
            rerun::archetypes::TextLog::new(msg).with_level(to_rerun_log_level(record.level()));
        let _ = rec.log(self.path.as_str(), &text);
        Ok(())
    }

    fn flush(&self) -> Result<(), Error> {
        Ok(())
    }
}
