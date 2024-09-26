use app::App;

mod app;
mod tui;

fn main() {
    let args = std::env::args();
    let mut app = App::init(args);
    println!("{:?}", app.list);
    app.run();
}
