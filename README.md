# Linux 
RUST_LOG=debug cargo run 
这表示：临时设置环境变量并执行 cargo run

# Windows 临时设置环境变量可以这样写：
## Powershell：
$env:RUST_LOG="debug";cargo run
## bat：
(set RUST_LOG=debug) && cargo run

#RUST_LOG 可以设置以下值
//levels of the logging
pub enum LogLevel {
    Error,         //error是日志分级的最高等级
    Warn,
    Info,
    Debug,
    Trace,         //trace是最低等级
}