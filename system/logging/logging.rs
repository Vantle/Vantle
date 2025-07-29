pub use log::{debug, error, info, warn};

pub fn configure(level: log::LevelFilter) {
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            use std::io::Write;

            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_else(|_| std::time::Duration::from_secs(0))
                .as_secs();

            let hours = (now % 86400) / 3600;
            let minutes = (now % 3600) / 60;
            let seconds = now % 60;

            let level_color = match record.level() {
                log::Level::Error => "\x1b[31m",
                log::Level::Warn => "\x1b[33m",
                log::Level::Info => "\x1b[32m",
                log::Level::Debug => "\x1b[34m",
                log::Level::Trace => "\x1b[35m",
            };

            writeln!(
                buf,
                "({:02}:{:02}:{:02}) {}{}\x1b[0m: {}",
                hours,
                minutes,
                seconds,
                level_color,
                record.level(),
                record.args()
            )
        })
        .filter_level(level)
        .init();
}
