use home::home_dir;
use tracing_appender::rolling;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{filter::EnvFilter, fmt, layer::SubscriberExt, Registry};

fn fca_home_str() -> String {
    let home = home_dir().expect("get home dir error");
    let buf = home.join(".jet");
    let str = buf.to_str().expect("to str error");
    str.to_owned()
}

pub fn init_log() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // 输出到控制台中
    let formatting_layer = fmt::layer().with_writer(std::io::stderr);
    let file_appender = rolling::daily(fca_home_str(), "fca.log");
    // let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer().with_ansi(false).with_writer(file_appender);
    // 注册
    Registry::default()
        .with(env_filter)
        .with(file_layer)
        .with(formatting_layer)
        .init();
}
