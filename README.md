# logforth-rerun

A simple [logforth](https://github.com/fast/logforth) `Append` that
will send logs to a rerun `RecordingStream`. This allows you to view
`log` records in the rerun visualizer and also save to a `.rrd` file.

> Forked from [tracing-rerun](https://github.com/therishidesai/tracing-rerun),
> swapping the `tracing` `Layer` for a `logforth` appender.

## Usage

```rust
use logforth::append;
use logforth::record::{Level, LevelFilter};
use logforth_rerun::RerunAppender;
use rerun::RecordingStreamBuilder;

let rec = RecordingStreamBuilder::new("my_app").save("logforth_export.rrd")?;

logforth::starter_log::builder()
    .dispatch(|d| d.filter(LevelFilter::MoreSevereEqual(Level::Info))
                   .append(append::Stderr::default()))
    .dispatch(|d| d.append(RerunAppender { rec: rec.clone(), path: "logs".into() }))
    .apply();

log::info!("hello from log, goes to stderr and Rerun");
```

See `examples/simple.rs` for a runnable version.
