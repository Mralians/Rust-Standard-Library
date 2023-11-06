#[macro_use]
extern crate log;

mod logger;
use logger::FileLogger;

use log::Level;
fn main() {
    FileLogger::init(Level::Info, "mralians.log").unwrap();
    info!("Helllo");
    error!("Error!");
}
