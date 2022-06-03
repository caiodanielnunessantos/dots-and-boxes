mod logic;
mod app;

use app::Player;
use sdl2::pixels::Color;

pub fn main() {
    let mut application = app::App::new(
        (1200, 700),
        50,
        Color::RGB(230, 245, 245),
        Color::RGB(64, 64, 64),
        (10, 8),
        Color::RGB(0, 0, 0),
        &[
            Player { initial: 'C', color: Color::GREEN},
            Player { initial: 'M', color: Color::BLUE},
            Player { initial: 'Y', color: Color::RGB(255, 127, 127)}
        ],
    );
    application.init();
}
