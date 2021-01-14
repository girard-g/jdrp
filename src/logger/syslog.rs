use log::LevelFilter;
use syslog::{Error, Facility};

pub fn init() -> Result<(), Error> {
    syslog::init_unix(Facility::LOG_USER, LevelFilter::Info)
}
