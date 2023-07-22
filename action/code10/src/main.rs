use std::env;
use std::path::Path;
use my_macro;
use crate::log::Logger;

mod log;
mod log_macro;
mod test_declarative;
mod test_procedural;


fn main() {
    // let logger = log_default!();
    // logger.i("default ...");

    info!("这是一条 info 日志");
    debug!("这是一条 debug 日志");
    warn!("这是一条 warn 日志");
    error!("这是一条 error 日志");
    fatal!("这是一条 fatal 日志");
}

