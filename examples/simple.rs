use anyhow::Result;
use logforth::append;
use logforth::record::{Level, LevelFilter};
use logforth_rerun::RerunAppender;
use rerun::RecordingStreamBuilder;

fn main() -> Result<()> {
    // Save to an .rrd file; use .connect_grpc(...) or .spawn() for a live viewer instead.
    let rec = RecordingStreamBuilder::new("my_app").save("logforth_export.rrd")?;

    logforth::starter_log::builder()
        .dispatch(|d| {
            d.filter(LevelFilter::MoreSevereEqual(Level::Info))
                .append(append::Stderr::default())
        })
        .dispatch(|d| {
            d.append(RerunAppender {
                rec: rec.clone(),
                path: "logs".into(),
            })
        })
        .apply();

    loop {
        log::error!("hello from log, goes to stderr and Rerun");
        log::info!(target: "db", "query finished (op=select rows=3)");
        log::warn!(target: "auth", "token expired");
        std::thread::sleep(std::time::Duration::from_millis(5000));
    }
}
