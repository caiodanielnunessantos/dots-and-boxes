mod logic;
mod app;
mod colors;

use app::Player;
use sdl2::{pixels::Color};

pub fn main() {
    let mut application = app::App::new(
        (1200, 700),
        50,
        colors::LightGray,
        colors::Black,
        (10, 8),
        colors::Black,
        &[
            Player { initial: 'C', color: Color::GREEN},
            Player { initial: 'M', color: Color::BLUE},
            Player { initial: 'Y', color: colors::LightPink}
        ],
    );
    application.init();
}
