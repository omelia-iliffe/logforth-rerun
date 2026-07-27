use logforth::record::{Level, Record};
use logforth::{Append, Diagnostic, Error};
use rerun::{RecordingStream, TextLogLevel};

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

/// A [`logforth`] appender that sends every log record to Rerun as a `TextLog`, logged to
/// `path` in the given [`RecordingStream`].
pub struct RerunAppender {
    pub rec: RecordingStream,
    /// Rerun entity path the records are logged under, e.g. `"logs"`.
    pub path: String,
}

impl std::fmt::Debug for RerunAppender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RerunAppender")
            .field("path", &self.path)
            .finish_non_exhaustive()
    }
}

impl Append for RerunAppender {
    fn append(&self, record: &Record, _diags: &[Box<dyn Diagnostic>]) -> Result<(), Error> {
        let msg = format!("{}: {}", record.target(), record.payload());
        let text =
            rerun::archetypes::TextLog::new(msg).with_level(to_rerun_log_level(record.level()));
        let _ = self.rec.log(self.path.as_str(), &text);
        Ok(())
    }

    fn flush(&self) -> Result<(), Error> {
        Ok(())
    }
}
