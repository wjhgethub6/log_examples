//Log 特征
// 例如，它定义了一个 Log 特征：

// pub trait Log: Sync + Send {
//     fn enabled(&self, metadata: &Metadata<'_>) -> bool;
//     fn log(&self, record: &Record<'_>);
//     fn flush(&self);
// }
// enabled 用于判断某条带有元数据的日志是否能被记录，它对于 log_enabled! 宏特别有用
// log 会记录 record 所代表的日志
// flush 会将缓存中的日志数据刷到输出中，例如标准输出或者文件中


// 日志宏
// log 还为我们提供了一整套标准的宏，用于方便地记录日志。
//trace!、debug!、info!、warn!、error!，这几个大家是否很眼熟，是的，
//它们跟我们上一章节提到的日志级别几乎一模一样，唯一的区别就是这里乱入了一个 trace!，
//它比 debug! 的日志级别还要低、记录的信息还要详细，
//这么说吧，如果你想事无巨细的追踪某个流程的所有信息，就可以用它了。

// use log::{info, trace, warn};
use log::{debug, error, log_enabled, info, Level, trace, warn};
#[warn(unused_imports)]

pub fn log_trace() {
    trace!("trace here 01");
    if true {
        trace!("trace here true");
    } else {
        trace!("trace here false");
    }
}

//log_enabled! 宏
use log::Level::Debug;
struct Data{
    x: usize,
    y: usize,
}

//log_enabled! 宏用于确定一条消息在当前模块中，对于给定的日志级别是否能够被记录
pub fn test_log_enabled() {
    // 判断能否记录 Debug 消息
    if log_enabled!(Debug) {
        let data = Data{x: 10, y: 10};
        // 下面的日志记录较为昂贵，因此我们先在前面判断了是否能够记录，能，才继续这里的逻辑
        debug!("expensive debug data: {} {}", data.x, data.y);
    }
    if log_enabled!(target: "Global", Debug) {
        let data =  Data{x: 10, y: 10};
        debug!(target: "Global", "expensive debug data: {} {}", data.x, data.y);
    }
}

//log! 宏就简单的多，它是一个通用的日志记录方式，因此需要我们手动指定日志级别：
pub fn test_log() {
    use log::{log};

    let data = (42, "Forty-two");
    let private_data = "private";

    log!(Level::Error, "Received errors: {}, {}", data.0, data.1);
    log!(target: "app_events", Level::Warn, "App warning: {}, {}, {}",
        data.0, data.1, private_data);
}

//log 还提供了 set_logger 函数用于设置日志库，set_max_level 用于设置最大日志级别

use log::set_max_level;
use log::LevelFilter;
fn main() {
    // 注意，env_logger 必须尽可能早的初始化
    env_logger::init();

    //RUST_LOG 始终使用 trace, 通过set_max_level可以动态修改level
    set_max_level(LevelFilter::Error);

    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }

    log_trace();
    test_log_enabled();
    test_log();
}
