use std::io;

use rand::Rng;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout},
    symbols::border,
    text::Text,
    widgets::{block::Position, Block, Borders, Paragraph, Widget},
    DefaultTerminal, Frame,
};

const SIZE: usize = 4;

#[derive(Default)]
pub struct App {
    grid: [[u32; SIZE]; SIZE],
    exit: bool,
}

impl App {
    pub fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut app = App::default();
        app.spawn();
        while !app.exit {
            terminal.draw(|frame| app.draw(frame))?;
            app.handle_input()?;
        }

        Ok(())
    }

    fn handle_input(&mut self) -> io::Result<()> {
        let key;
        if let event::Event::Key(k) = event::read()? {
            key = k;
        } else {
            return Ok(());
        }
        if let KeyCode::Char(ch) = key.code {
            let mut spawn = true;
            match ch {
                'q' => self.exit = true,
                'h' => self.move_left(),
                'j' => self.move_down(),
                'k' => self.move_up(),
                'l' => self.move_right(),
                _ => spawn = false,
            }
            if spawn {
                self.spawn();
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn spawn(&mut self) -> bool {
        let mut indexes: Vec<(usize, usize)> = vec![];
        for y in 0..SIZE {
            for x in 0..SIZE {
                if self.grid[y][x] == 0 {
                    indexes.push((y, x));
                }
            }
        }
        if indexes.is_empty() {
            return false;
        }
        let index = rand::thread_rng().gen_range(0..indexes.len());
        let (r, c) = indexes.remove(index);
        let num = if rand::thread_rng().gen_range(0..10) == 4 {
            4
        } else {
            2
        };
        self.grid[r][c] = num;

        true
    }

    fn move_left(&mut self) {
        for y in 0..SIZE {
            let mut target = 0;
            for x in 0..SIZE {
                if self.grid[y][x] == 0 {
                    continue;
                }
                let value = self.grid[y][x];
                self.grid[y][x] = 0;
                self.grid[y][target] = value;
                target += 1;
            }
        }

        for y in 0..SIZE {
            let mut target = 0;
            while target < SIZE - 1 {
                if self.grid[y][target] != 0 && self.grid[y][target] == self.grid[y][target + 1] {
                    self.grid[y][target] *= 2;
                    self.grid[y][target + 1] = 0;
                    target += 1;
                }
                target += 1;
            }
        }

        for y in 0..SIZE {
            let mut target = 0;
            for x in 0..SIZE {
                if self.grid[y][x] == 0 {
                    continue;
                }
                let value = self.grid[y][x];
                self.grid[y][x] = 0;
                self.grid[y][target] = value;
                target += 1;
            }
        }
    }

    fn move_right(&mut self) {
        for y in 0..SIZE {
            let mut target = SIZE - 1;
            for x in (0..SIZE).rev() {
                if self.grid[y][x] != 0 {
                    let value = self.grid[y][x];
                    self.grid[y][x] = 0;
                    self.grid[y][target] = value;
                    target = target.saturating_sub(1);
                }
            }
        }

        for y in 0..SIZE {
            let mut target = SIZE - 1;
            while target > 0 {
                if self.grid[y][target] != 0 && self.grid[y][target] == self.grid[y][target - 1] {
                    self.grid[y][target] *= 2;
                    self.grid[y][target - 1] = 0;
                    target = target.saturating_sub(1);
                }
                target = target.saturating_sub(1);
            }
        }

        for y in 0..SIZE {
            let mut target = SIZE - 1;
            for x in (0..SIZE).rev() {
                if self.grid[y][x] != 0 {
                    let value = self.grid[y][x];
                    self.grid[y][x] = 0;
                    self.grid[y][target] = value;
                    target = target.saturating_sub(1);
                }
            }
        }
    }

    fn move_up(&mut self) {
        for x in 0..SIZE {
            let mut target = 0;
            for y in 0..SIZE {
                if self.grid[y][x] == 0 {
                    continue;
                }
                let value = self.grid[y][x];
                self.grid[y][x] = 0;
                self.grid[target][x] = value;
                target += 1;
            }
        }

        for x in 0..SIZE {
            let mut target = 0;
            while target < SIZE - 1 {
                if self.grid[target][x] != 0 && self.grid[target][x] == self.grid[target + 1][x] {
                    self.grid[target][x] *= 2;
                    self.grid[target + 1][x] = 0;
                    target += 1;
                }
                target += 1;
            }
        }

        for x in 0..SIZE {
            let mut target = 0;
            for y in 0..SIZE {
                if self.grid[y][x] == 0 {
                    continue;
                }
                let value = self.grid[y][x];
                self.grid[y][x] = 0;
                self.grid[target][x] = value;
                target += 1;
            }
        }
    }

    fn move_down(&mut self) {
        for x in 0..SIZE {
            let mut target = SIZE - 1;
            for y in (0..SIZE).rev() {
                if self.grid[y][x] == 0 {
                    continue;
                }
                let value = self.grid[y][x];
                self.grid[y][x] = 0;
                self.grid[target][x] = value;
                target = target.saturating_sub(1);
            }
        }

        for x in 0..SIZE {
            let mut target = SIZE - 1;
            while target > 0 {
                if self.grid[target][x] != 0 && self.grid[target][x] == self.grid[target - 1][x] {
                    self.grid[target][x] *= 2;
                    self.grid[target - 1][x] = 0;
                    target = target.saturating_sub(1);
                }
                target = target.saturating_sub(1);
            }
        }

        for x in 0..SIZE {
            let mut target = SIZE - 1;
            for y in (0..SIZE).rev() {
                if self.grid[y][x] == 0 {
                    continue;
                }
                let value = self.grid[y][x];
                self.grid[y][x] = 0;
                self.grid[target][x] = value;
                target = target.saturating_sub(1);
            }
        }
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let temp = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(0),         // Space above the widget
                Constraint::Length(5 * 4 + 2), // Height of the widget (centered area)
                Constraint::Min(0),            // Space below the widget
            ])
            .split(area);

        let rec = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(0),         // Space on the left
                Constraint::Length(9 * 4 + 4), // Centered widget area
                Constraint::Min(0),            // Space on the right
            ])
            .split(temp[1]);

        Block::bordered()
            .border_set(border::DOUBLE)
            .title("2048")
            .title_alignment(Alignment::Center)
            .render(rec[1], buf);
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                if val == 0 {
                    continue;
                }
                let temp = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(1 + y as u16 * 5), // Space above the widget
                        Constraint::Length(5),                // Height of the widget
                        Constraint::Min(0),                   // Fill the remaining space
                    ])
                    .split(area);

                let rec = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Length(2 + x as u16 * 9), // Space on the left
                        Constraint::Length(9),                // Centered widget area
                        Constraint::Min(0),                   // Space on the right
                    ])
                    .split(temp[1]);

                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_set(border::ROUNDED);

                let number = Paragraph::new(format!("\n{}", val));
                number
                    .alignment(Alignment::Center)
                    .block(block)
                    .render(rec[1], buf);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_left_works() {
        let mut game = App::default();
        game.grid[2][1] = 4;
        game.grid[2][2] = 2;
        game.grid[2][3] = 2;
        game.move_left();
        assert_eq!(game.grid[2][1], 4);
        assert_eq!(game.grid[2][0], 4);
    }

    #[test]
    fn move_right_works() {
        let mut game = App::default();
        game.grid[2][1] = 4;
        game.grid[2][2] = 2;
        game.grid[2][3] = 2;
        game.move_right();
        assert_eq!(game.grid[2][3], 4);
        assert_eq!(game.grid[2][2], 4);
    }

    #[test]
    fn move_up_works() {
        let mut game = App::default();
        game.grid[0][2] = 4;
        game.grid[2][2] = 2;
        game.grid[3][2] = 2;
        println!("{:?}", game.grid);
        game.move_up();
        println!("{:?}", game.grid);
        assert_eq!(game.grid[0][2], 4);
        assert_eq!(game.grid[1][2], 4);
    }

    #[test]
    fn move_down_works() {
        let mut game = App::default();
        game.grid[0][2] = 4;
        game.grid[2][2] = 2;
        game.grid[3][2] = 2;
        println!("{:?}", game.grid);
        game.move_down();
        println!("{:?}", game.grid);
        assert_eq!(game.grid[2][2], 4);
        assert_eq!(game.grid[3][2], 4);
    }
}
