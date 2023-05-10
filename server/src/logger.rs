use std::{num::NonZeroU8, panic, thread};

use time::format_description::well_known::{
    self,
    iso8601::{Config, EncodedConfig, TimePrecision},
};
use tracing::{error, level_filters::LevelFilter};
use tracing_subscriber::fmt::time::LocalTime;

const MAX_LEVEL: LevelFilter = LevelFilter::INFO;
const CONFIG: EncodedConfig = Config::DEFAULT
    .set_time_precision(TimePrecision::Second {
        decimal_digits: NonZeroU8::new(3),
    })
    .encode();

pub fn init() {
    tracing_subscriber::fmt()
        .with_timer(LocalTime::new(well_known::Iso8601::<CONFIG>))
        .with_max_level(MAX_LEVEL)
        .init();

    // catch panic and log them using tracing instead of default output to StdErr
    panic::set_hook(Box::new(|info| {
        let thread = thread::current();
        let thread = thread.name().unwrap_or("unknown");

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &**s,
                None => "Box<Any>",
            },
        };

        let backtrace = backtrace::Backtrace::new();

        match info.location() {
            Some(location) => {
                // without backtrace
                if msg.starts_with("notrace - ") {
                    error!(
                        target: "panic", "thread '{}' panicked at '{}': {}:{}",
                        thread,
                        msg.replace("notrace - ", ""),
                        location.file(),
                        location.line()
                    );
                }
                // with backtrace
                else {
                    error!(
                        target: "panic", "thread '{}' panicked at '{}': {}:{}\n{:?}",
                        thread,
                        msg,
                        location.file(),
                        location.line(),
                        backtrace
                    );
                }
            }
            None => {
                // without backtrace
                if msg.starts_with("notrace - ") {
                    error!(
                        target: "panic", "thread '{}' panicked at '{}'",
                        thread,
                        msg.replace("notrace - ", ""),
                    );
                }
                // with backtrace
                else {
                    error!(
                        target: "panic", "thread '{}' panicked at '{}'\n{:?}",
                        thread,
                        msg,
                        backtrace
                    );
                }
            }
        }
    }));
}
