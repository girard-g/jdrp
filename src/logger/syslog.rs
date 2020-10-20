use syslog::{self, Facility, Error};
use log::LevelFilter;

pub fn init() -> Result<(), Error> {
    syslog::init_unix(Facility::LOG_USER, LevelFilter::Info)
}
