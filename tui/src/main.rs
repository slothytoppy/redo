use app::App;
use tracing_subscriber::FmtSubscriber;

mod app;
mod cursor;
mod event;
mod renderer;
mod tui;
mod viewport;

fn main() {
    let appender = tracing_appender::rolling::never(".", "log");
    let (appender, _guard) = tracing_appender::non_blocking(appender);
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .with_writer(appender)
        .with_ansi(false)
        .finish();
    let args = std::env::args();
    let _ = tracing::subscriber::set_global_default(subscriber);
    let mut app = App::init(args);

    app.run();
}
