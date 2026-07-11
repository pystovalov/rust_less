use log::{debug, error, info, warn};
fn main() {
    env_logger::init();
    info!("This is an info message");
    warn!("This is a warning");
    error!("This is an error");
    debug!("This is a debug message");
}
