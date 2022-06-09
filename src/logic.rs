use super::app::Line;

pub struct DotsAndBoxesState {
    players_count: i32,
    height: i32,
    width: i32,
    pub next_player: i32,
    horizontal_lines: Box<[i32]>,
    vertical_lines: Box<[i32]>,
    squares: Box<[i32]>,
}

impl std::fmt::Debug for DotsAndBoxesState {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Dots and Boxes State")
            .field("Players Count", &self.players_count)
            .field("Height", &self.height)
            .field("Width", &self.width)
            .field("Next Player", &self.next_player)
            .finish()
    }
}

impl DotsAndBoxesState {
    pub fn create(players_count: i32, height: i32, width: i32) -> DotsAndBoxesState {
        let mut horizontal_lines = Vec::new();
        horizontal_lines.resize(width as usize * (height + 1) as usize, 0);
        let mut vertical_lines = Vec::new();
        vertical_lines.resize(height as usize * (width + 1) as usize, 0);
        let mut squares = Vec::new();
        squares.resize(width as usize * height as usize, 0);
        DotsAndBoxesState {
            players_count,
            height,
            width,
            next_player: 1,
            horizontal_lines: horizontal_lines.into_boxed_slice(),
            vertical_lines: vertical_lines.into_boxed_slice(),
            squares: squares.into_boxed_slice(),
        }
    }
    pub fn play_line(&mut self, line: Option<Line>) {
        let s = |x: i32, y: i32| (y * self.width + x) as usize;
        let h = |x: i32, y: i32| (y * self.width + x) as usize;
        let v = |x: i32, y: i32| (y * (self.width + 1) + x) as usize;

        match line {
            Some(Line::Horizontal(x, y)) => {
                if self.horizontal_lines[h(x, y)] == 0 {
                    self.horizontal_lines[h(x, y)] = self.next_player;
                    let mut play_again = false;
                    if y > 0
                        && self.horizontal_lines[h(x, y - 1)] != 0
                        && self.vertical_lines[v(x, y - 1)] != 0
                        && self.vertical_lines[v(x + 1, y - 1)] != 0
                    {
                        play_again = true;
                        self.squares[s(x, y - 1)] = self.next_player;
                    }
                    if y < self.height
                        && self.horizontal_lines[h(x, y + 1)] != 0
                        && self.vertical_lines[v(x, y)] != 0
                        && self.vertical_lines[v(x + 1, y)] != 0
                    {
                        play_again = true;
                        self.squares[s(x, y)] = self.next_player;
                    }
                    if play_again { () } else {
                        if self.next_player == self.players_count {
                            self.next_player = 1;
                        } else {
                            self.next_player += 1;
                        }
                    }
                }
            },
            Some(Line::Vertical(x, y)) => {
                if self.vertical_lines[v(x, y)] == 0 {
                    self.vertical_lines[v(x, y)] = self.next_player;
                    let mut play_again = false;
                    if x > 0
                        && self.vertical_lines[v(x - 1, y)] != 0
                        && self.horizontal_lines[h(x - 1, y)] != 0
                        && self.horizontal_lines[h(x - 1, y + 1)] != 0
                    {
                        self.squares[s(x - 1, y)] = self.next_player;
                        play_again = true;
                    }
                    if x < self.width
                        && self.vertical_lines[v(x + 1, y)] != 0
                        && self.horizontal_lines[h(x, y)] != 0
                        && self.horizontal_lines[h(x, y + 1)] != 0
                    {
                        self.squares[s(x, y)] = self.next_player;
                        play_again = true;
                    }
                    if play_again { () } else {
                        if self.next_player == self.players_count {
                            self.next_player = 1;
                        } else {
                            self.next_player += 1;
                        }
                    }
                }
            },
            None => ()
        }
    }
    pub fn print_to_terminal(&self) {
        let str_size = (self.width as usize * 2 + 1) * (self.height as usize * 2 + 1);
        let mut out = String::new();
        for element in 0..str_size {
            let canvas_x = element % (self.width as usize * 2 + 1);
            let canvas_y = element / (self.width as usize * 2 + 1);
            if canvas_x == 0 {
                out.push('\n');
            }
            if canvas_x % 2 == 0 && canvas_y % 2 == 0 {
                out.push_str(" * ");
            } else if canvas_x % 2 == 1 && canvas_y % 2 == 0 {
                let hlx = canvas_x / 2;
                let hly = canvas_y / 2;
                let index = hlx + hly * self.width as usize;
                if self.horizontal_lines[index] != 0 {
                    out.push_str(" A ");
                } else {
                    out.push_str(" h ");
                }
            } else if canvas_x % 2 == 0 && canvas_y % 2 == 1 {
                let vlx = canvas_x / 2;
                let vly = canvas_y / 2;
                let index = vlx + vly * (self.width as usize + 1);
                if self.vertical_lines[index] != 0 {
                    out.push_str(" B ");
                } else {
                    out.push_str(" v ");
                }
            } else {
                let sqx = canvas_x / 2;
                let sqy = canvas_y / 2;
                let index = sqx + sqy * self.width as usize;
                if self.squares[index] != 0 {
                    out.push_str(" S ");
                } else {
                    out.push_str(" s ");
                }
            }
        }
        print!("{}", &out);
    }
    pub fn get_raw_buffers(&self) -> (&Box<[i32]>, &Box<[i32]>, &Box<[i32]>) {
        return (
            &self.horizontal_lines,
            &self.vertical_lines,
            &self.squares,
        )
    }
}
