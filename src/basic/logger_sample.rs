//!
//! Basic use log, print application running log infomation.
//! Log framwork : log ,tracing.
//! Print log level 、trace.
//!

use env_logger;
use log::{self, debug, info, trace};
use rand::{self, random};
/**
 * logger print
 */
pub(crate) fn logger_print() {
    // init logger and setting Level Debug
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Debug)
        .init();

    // 或
    // env_logger::init();

    let n = random::<i32>();

    info!("logger is info,random n :{}", n);

    let m = random::<char>();
    trace!("logger trace is {}", m);

    let x = 32;
    debug!("this is debug info.{}", x);

    log::info!("This will be logged to the file");

    let y =add_one(x);
    
    log::info!("add one result is {}",y);
}

fn add_one(num: i32) -> i32 {
    info!("add_one called with {}", num);
    num + 1
}

// pub mod  yak_shave;
use crate::{add, basic::yak_shave};
use tracing;
use tracing_subscriber::FmtSubscriber;
/**
 * trace log print
 */
pub(crate) fn tracing_sample() {
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    tracing::info!(number_of_yaks, "preparing to shave yaks");

    let number_shaved = yak_shave::shave_all(number_of_yaks);
    tracing::info!(
        all_yaks_shaved = number_shaved == number_of_yaks,
        "yak shaving completed."
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn it_adds_one() {
        init();

        info!("can log from the test too");
        assert_eq!(3, add_one(2));
    }

    #[test]
    fn it_handles_negative_numbers() {
        init();

        info!("logging from another test");
        assert_eq!(-7, add_one(-8));
    }
}
