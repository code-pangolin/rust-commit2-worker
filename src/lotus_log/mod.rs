use anyhow::{Ok, Result};
use colorful::{Color, Colorful};
use log::{set_max_level, Level, Log};

pub struct LotusLog {
    name: String,
    level: Level,
    time_format: Option<String>,
}

impl Log for LotusLog {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let now = chrono::Local::now();

        let level_color = match record.metadata().level() {
            Level::Error => Color::Red,
            Level::Warn => Color::Yellow,
            Level::Info => Color::White,
            Level::Debug => Color::Green,
            Level::Trace => Color::Green,
        };

        let output = if let Some(t) = &self.time_format {
            format!(
                "{}\t{}\t{}\t{}",
                now.format(t),
                record.level().as_str().gradient(level_color),
                &self.name,
                record.args()
            )
        } else {
            format!("{}\t{}\t{}", record.level(), &self.name, record.args())
        };

        print!("{}\n", output);
    }

    fn flush(&self) {}
}

impl LotusLog {
    pub fn new(name: String, level: Level) -> Self {
        LotusLog {
            name,
            level,
            time_format: None,
        }
    }

    /// See the [`chrono::format::strftime`] module
    pub fn set_time_format(&mut self, format: String) {
        self.time_format = Some(format);
    }

    pub fn init(&'static self) -> Result<()> {
        set_max_level(self.level.to_level_filter());
        // log::set_boxed_logger(Box::new(self))?;
        log::set_logger(self)?;
        Ok(())
    }
}