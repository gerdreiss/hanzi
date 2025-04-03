use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::Appender;
use log4rs::config::Config;
use log4rs::config::Root;
use log4rs::config::runtime::ConfigErrors;
use log4rs::encode::pattern::PatternEncoder;
use std::env;
use std::str::FromStr;

use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub(crate) enum LoggingError {
    #[error("Configuring log file location failed")]
    IO(#[from] std::io::Error),
    #[error("Configuring logging failed")]
    Config(#[from] ConfigErrors),
    #[error("Setting logger failed")]
    Set(#[from] log::SetLoggerError),
}

pub(crate) fn init() {
    match try_init() {
        Ok(_) => (),
        Err(err) => println!("{:?}", err),
    }
}

pub(crate) fn try_init() -> Result<(), LoggingError> {
    let log_level = get_log_level();
    let logfile_path = get_logfile_path();

    let log_pattern = "{d} {l} - {m}{n}";

    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(log_pattern)))
        .build();
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(log_pattern)))
        .build(logfile_path)?;

    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console)))
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("console").appender("logfile").build(log_level))?;

    log4rs::init_config(config)?;

    Ok(())
}

fn get_log_level() -> log::LevelFilter {
    let log_level_str = env::var("RUST_LOG")
        .map(|v| v.to_uppercase())
        .unwrap_or("INFO".to_string());
    log::LevelFilter::from_str(&log_level_str).unwrap_or(log::LevelFilter::Info)
}

fn get_logfile_path() -> String {
    let home_dir = env::var("HOME").expect("$HOME environment variable to exist");
    let hanzi_dir = format!("{}/.hanzi", home_dir);

    if !std::path::Path::new(&hanzi_dir).exists() {
        std::fs::create_dir_all(&hanzi_dir).expect("Successful folder creation");
    }

    format!("{}/app.log", hanzi_dir)
}
