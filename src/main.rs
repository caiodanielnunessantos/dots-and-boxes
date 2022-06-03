mod logic;
mod app;

use app::Player;
use sdl2::pixels::Color;

pub fn main() {
    let mut application = app::App::new(
        (800, 600),
        80,
        Color::RGB(230, 230, 230),
        Color::RGB(64, 64, 64),
        (5, 4),
        Color::RGB(0, 0, 0),
        &[
            Player { initial: 'C', color: Color::GREEN},
            Player { initial: 'M', color: Color::BLUE},
        ],
    );
    application.init();
}
