#[cfg(feature = "env_logger")]
pub use self::env_logger::init_logger;
#[cfg(feature = "file-logger")]
pub use self::file_logger::init_file_logger;
#[cfg(feature = "tracing")]
pub use self::tracing::init_tracing_logger;

#[cfg(feature = "env_logger")]
mod env_logger {
    use chrono::{Local, SecondsFormat};
    use env_logger::fmt::style::Color;
    use log::{Level, LevelFilter};
    use std::io::Write;

    pub fn init_logger() {
        env_logger::builder()
            .filter_level(LevelFilter::Debug)
            .format(|buf, record| {
                let color = match record.level() {
                    Level::Warn => Some(Color::Ansi256(215_u8.into())),
                    Level::Error => Some(Color::Ansi256(203_u8.into())),
                    _ => None,
                };

                let level_style = buf.default_level_style(record.level());
                let reset = level_style.render_reset();
                let render = level_style.fg_color(color).render();
                writeln!(
                    buf,
                    "{render}[{}|{}|line:{}]: {}{reset}",
                    Local::now().to_rfc3339_opts(SecondsFormat::Millis, false),
                    record.level(),
                    record.line().unwrap_or(0),
                    record.args()
                )
            })
            .init();
    }
}

#[cfg(feature = "file-logger")]
mod file_logger {
    use flexi_logger::{
        colored_opt_format, opt_format, Cleanup, Criterion, Duplicate, FileSpec, FlexiLoggerError, LoggerHandle,
        Naming, WriteMode,
    };
    use log::LevelFilter;
    pub fn init_file_logger() -> Result<LoggerHandle, FlexiLoggerError> {
        // WriteMode::BufferAndFlush 需要主进程保持LoggerHandle存活, let _l = init_file_logger();
        // 不能使用 let _ = ...
        flexi_logger::Logger::try_with_env_or_str(LevelFilter::Info.as_str())?
            .log_to_file(
                FileSpec::default()
                    .basename("someservice")
                    .directory("./logs"),
            )
            .rotate(
                Criterion::Size(100_u64 * 1024_u64.pow(2_u32)),
                Naming::NumbersDirect,
                Cleanup::KeepLogFiles(7),
            )
            .append()
            .write_mode(WriteMode::BufferAndFlush)
            .duplicate_to_stdout(Duplicate::Info)
            .format_for_files(opt_format)
            .format_for_stdout(colored_opt_format)
            .start()
    }
}

#[cfg(feature = "tracing")]
mod tracing {
    use tracing_subscriber::fmt::writer::MakeWriterExt;
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
    pub fn init_tracing_logger() -> tracing_appender::non_blocking::WorkerGuard {
        // 使用 time 库
        // use tracing_subscriber::fmt::time::OffsetTime;
        // use time::macros::format_description;
        // let offset = time::UtcOffset::current_local_offset().expect("should get local offset!");
        // let timer = OffsetTime::new(
        //     offset,
        //     format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"));

        // 使用 chrono 库
        use tracing_subscriber::fmt::time::ChronoLocal;
        let timer = ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f%:z".to_string());

        // tracing_appender 只能按日期切割
        // use tracing_appender::rolling::Rotation;
        // let file_appender = tracing_appender::rolling::Builder::new()
        //     .filename_prefix("thisapp")
        //     .max_log_files(10)
        //     .rotation(Rotation::DAILY)
        //     .build(".").unwrap();

        // tracing_rolling_file 按文件大小切割
        use tracing_rolling_file::{RollingConditionBase, RollingFileAppender};
        let file_appender = RollingFileAppender::new(
            "./thisapp",
            RollingConditionBase::new().max_size(100_u64 * 1024_u64.pow(2_u32)),
            10,
        )
        .unwrap();

        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        let file_layer = fmt::Layer::new()
            .with_timer(timer.clone())
            .with_ansi(false)
            .with_writer(non_blocking.with_max_level(tracing::Level::INFO));
        let console_layer = fmt::Layer::new().with_timer(timer);
        tracing_subscriber::registry()
            .with(file_layer)
            .with(console_layer)
            .init();
        _guard
    }
}
