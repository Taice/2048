use std::io;

use app::App;

mod app;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::run(&mut terminal);
    ratatui::restore();
    app_result
}
