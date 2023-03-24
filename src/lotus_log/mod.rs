use anyhow::{Ok, Result};
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
        let output = if let Some(t) = &self.time_format {
            format!(
                "{} {}: {} - {}",
                now.format(t),
                &self.name,
                record.level(),
                record.args()
            )
        } else {
            format!("{}: {} - {}", &self.name, record.level(), record.args())
        };

        print!("{}", output);
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
