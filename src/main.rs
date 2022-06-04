mod logic;
mod app;
mod colors;

use app::Player;

pub fn main() {
    let mut application = app::App::new(
        (1200, 700),
        50,
        colors::LIGHT_GRAY,
        colors::BLACK,
        (10, 8),
        colors::BLACK,
        &[
            Player { initial: 'C', color: colors::GREEN},
            Player { initial: 'M', color: colors::BLUE},
            Player { initial: 'Y', color: colors::LIGHT_PINK}
        ],
    );
    application.init();
}
