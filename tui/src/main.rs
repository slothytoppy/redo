use app::App;

mod app;
mod cursor;
mod renderer;
mod tui;
mod viewport;

fn main() {
    let args = std::env::args();
    let mut app = App::init(args);

    println!("{:?}", app.collection);
    app.run();
}
