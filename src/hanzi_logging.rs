use std::env;
use std::fs::File;
use std::str::FromStr;
use std::time::SystemTime;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub(crate) enum LoggingError {
    #[error("Configuring log file location failed")]
    IO(#[from] std::io::Error),
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
    let logfile = get_logfile_path()?;

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .chain(logfile)
        .apply()?;

    Ok(())
}

fn get_log_level() -> log::LevelFilter {
    let log_level_str = env::var("RUST_LOG")
        .map(|v| v.to_uppercase())
        .unwrap_or("INFO".to_string());
    log::LevelFilter::from_str(&log_level_str).unwrap_or(log::LevelFilter::Info)
}

fn get_logfile_path() -> std::io::Result<File> {
    let home_dir = env::var("HOME").expect("$HOME environment variable to exist");
    let hanzi_dir = format!("{}/.hanzi", home_dir);

    if !std::path::Path::new(&hanzi_dir).exists() {
        std::fs::create_dir_all(&hanzi_dir).expect("Successful folder creation");
    }

    let logfile_path = format!("{}/app.log", hanzi_dir);
    fern::log_file(&logfile_path)
}
