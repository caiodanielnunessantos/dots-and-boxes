use super::logic::DotsAndBoxesState;

use std::time::{Duration, Instant};

use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

#[derive(Clone, Copy, Debug)]
pub struct Player {
    pub initial: char,
    pub color: Color,
}

#[derive(Debug)]
pub enum Line {
    Horizontal(i32, i32),
    Vertical(i32, i32),
}

pub struct App {
    window_size: (i32, i32),
    line_size: i32,
    background_color: Color,
    dot_color: Color,
    players: Box<[Player]>,
    state: Box<DotsAndBoxesState>,
    board_size: (i32, i32),
    hover: Option<Line>,
    hover_color: Color,
}

impl App {
    pub fn new(
        window_size: (i32, i32),
        line_size: i32,
        background_color: Color,
        dot_color: Color,
        board_size: (i32, i32),
        hover_color: Color,
        players: &[Player],
    ) -> Self {
        let state = DotsAndBoxesState::create(players.len() as i32, board_size.1, board_size.0);
        App {
            window_size,
            line_size,
            background_color,
            dot_color,
            players: players.into(),
            state: state.into(),
            board_size,
            hover: None,
            hover_color,
        }
    }
    pub fn init(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(
                "Dots And Boxes",
                self.window_size.0 as u32,
                self.window_size.1 as u32,
            )
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        let mut event_pump = sdl_context.event_pump().unwrap();

        'running: loop {
            let now = Instant::now();

            for event in event_pump.poll_iter() {
                match event {
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::Quit { .. } => break 'running,
                    Event::MouseMotion { x, y, .. } => {
                        self.hover = self.pos_to_line((x, y));
                    },
                    Event::MouseButtonDown { x, y, .. } => {
                        self.state.play_line(self.pos_to_line((x, y)).unwrap_or(Line::Horizontal(0, 0)))
                    }
                    _ => {}
                }
            }

            self.draw(&mut canvas);

            if let Some(remaining_time) =
                (Duration::from_nanos(1_000_000_000) / 75).checked_sub(now.elapsed())
            {
                std::thread::sleep(remaining_time);
            }
        }
    }
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.background_color);
        canvas.clear();

        let start_x = (self.window_size.0 - self.line_size * self.board_size.0) / 2;
        let start_y = (self.window_size.1 - self.line_size * self.board_size.1) / 2;

        let rect_line = |line: &Line| match line {
            Line::Horizontal(x, y) => {
                let center_x = start_x + self.line_size * x;
                let center_y = start_y + self.line_size * y;
                Rect::new(
                    center_x as i32,
                    center_y as i32 - 2,
                    self.line_size as u32,
                    5,
                )
            }
            Line::Vertical(x, y) => {
                let center_x = start_x + self.line_size * x;
                let center_y = start_y + self.line_size * y;
                Rect::new(
                    center_x as i32 - 2,
                    center_y as i32 - 2,
                    5,
                    self.line_size as u32,
                )
            }
        };

        let draw_square = |x: i32, y: i32| {
            let center_x = start_x + self.line_size * x;
            let center_y = start_y + self.line_size * y;
            Rect::new(
                center_x as i32 + 8,
                center_y as i32 + 8,
                self.line_size as u32 - 16,
                self.line_size as u32 - 16,
            )
        };

        canvas.set_draw_color(self.hover_color);
        if let Some(hover) = &self.hover {
            canvas.fill_rect(rect_line(hover)).unwrap();
        };

        let (horizontal_lines, vertical_lines, squares) = self.state.get_raw_buffers();

        for (number, square) in squares.into_iter().enumerate() {
            if *square != 0 {
                let x = number as i32 % self.board_size.0;
                let y = number as i32 / self.board_size.0;
                canvas.set_draw_color(self.players[*square as usize - 1].color);
                canvas.fill_rect(draw_square(x, y)).unwrap();
            }
        }

        for (number, line) in horizontal_lines.into_iter().enumerate() {
            if *line != 0 {
                let x = number as i32 % self.board_size.0;
                let y = number as i32 / self.board_size.0;
                canvas.set_draw_color(self.players[*line as usize - 1].color);
                canvas.fill_rect(rect_line(&Line::Horizontal(x, y))).unwrap();
            }
        }

        for (number, line) in vertical_lines.into_iter().enumerate() {
            if *line != 0 {
                let x = number as i32 % (self.board_size.0 + 1);
                let y = number as i32 / (self.board_size.0 + 1);
                canvas.set_draw_color(self.players[*line as usize - 1].color);
                canvas.fill_rect(rect_line(&Line::Vertical(x, y))).unwrap();
            }
        }

        canvas.set_draw_color(self.dot_color);
        for x in 0..self.board_size.0 + 1 {
            for y in 0..self.board_size.1 + 1 {
                let center_x = start_x + self.line_size * x;
                let center_y = start_y + self.line_size * y;
                canvas
                    .fill_rect(Rect::from_center((center_x as i32, center_y as i32), 5, 5))
                    .unwrap();
            }
        }

        canvas.present();
    }
    fn pos_to_line(&self, pos: (i32, i32)) -> Option<Line> {
        let (width, height) = self.board_size;
        let line_size = self.line_size as i32;
        let (windows_x, windows_y) = self.window_size;

        let start_x = (windows_x - line_size * width) / 2;
        let start_y = (windows_y - line_size * height) / 2;
        let square_x = (pos.0 - start_x) / line_size;
        let square_y = (pos.1 - start_y) / line_size;
        let inner_square_x = ((pos.0 - start_x) % line_size) - line_size / 2;
        let inner_square_y = ((pos.1 - start_y) % line_size) - line_size / 2;

        match (inner_square_x, inner_square_y) {
            (xx, yy) if (xx >= yy) && (xx + yy <= 0) => {
                if (square_x < 0) || (square_y < 0) || (square_x > width - 1) || (square_y > height)
                {
                    None
                } else {
                    Some(Line::Horizontal(square_x, square_y))
                }
            }
            (xx, yy) if (xx <= yy) && (xx + yy <= 0) => {
                if (square_x < 0) || (square_y < 0) || (square_x > width) || (square_y > height - 1)
                {
                    None
                } else {
                    Some(Line::Vertical(square_x, square_y))
                }
            }
            (xx, yy) if (xx >= yy) && (xx + yy >= 0) => {
                if (square_x + 1 < 0)
                    || (square_y < 0)
                    || (square_x + 1 > width)
                    || (square_y > height - 1)
                {
                    None
                } else {
                    Some(Line::Vertical(square_x + 1, square_y))
                }
            }
            (xx, yy) if (xx <= yy) && (xx + yy >= 0) => {
                if (square_x < 0)
                    || (square_y + 1 < 0)
                    || (square_x > width - 1)
                    || (square_y + 1 > height)
                {
                    None
                } else {
                    Some(Line::Horizontal(square_x, square_y + 1))
                }
            }
            _ => None,
        }
    }
}
