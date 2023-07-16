mod log;
mod test;

use crate::log::{Logger, LogLevel};

fn main() {
    let logger = Logger::new(Some("log.txt".to_owned()), LogLevel::Warning, true, true);
    logger.d("这是一条debug信息");
    logger.i("这是一条info信息");
    logger.w("这是一条warning信息");
    logger.e("这是一条error信息");
    logger.f("这是一条fatal信息");
}