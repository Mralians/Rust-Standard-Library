use env_logger;
use log::Level;

use std::panic;

#[macro_use]
extern crate log;

fn expensive_operation(_name: &str) -> String {
    trace!("Starting an expensive operation");
    let data = "Imagine this is a very very expensive task".to_owned();
    trace!("Finishid the expensive operation!");
    data
}
fn main() {
    // Custom panic
    panic::set_hook(Box::new(|e| {
        println!("Oh noes, something went wrong D:");
        println!("{e:?}");
    }));
    // env_logger's priority levels are:
    // error > warn > info > debug > trace
    env_logger::init();
    // All logging calls log! in the background
    log!(Level::Debug, "env_logger has been initialized");
    // There are convenience macros for every logging level however
    info!("The program has started!");

    // A log's target is its parent module per default
    // ('logging' in out case, as we're in a binary)
    // We can override this target however:
    // RUST_LOG="logging=trace"
    // RUST_LOG="extra_info=trace"
    // RUST_LOG="extra_info=trace,loggin=warn"
    // RUST_LOG="debug/expensive" --> debug level with expensive regex
    // RUST_LOG="logging=debug/expensive" --> logging target with debug level and expensive regex
    info!(target: "extra_info", "This is additional info that will only show if you \
          activate info level logging for thje extra_info target");
    warn!("Something that requires your attention happened!");
    // Only execute code if logging level is activate
    if log_enabled!(Level::Debug) {
        let name = "mralians";
        trace!("fn expensive_operation({})", name);
        let data = expensive_operation(name);
        debug!("The expensive operation retured: \"{}\" ", data);
    }
    error!("Something terrible happened!");
    panic!("A thing broke");
}
