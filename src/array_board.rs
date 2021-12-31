use anyhow::{Result};
use crossterm::{
    cursor::MoveTo,
    style::{style, Attribute, Color, PrintStyledContent},
    QueueableCommand,
};

use std::io::{stdout, Write};

const HEIGHT: usize = 6;
const WIDTH: usize = 7;

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Player1,
    Player2,
    Empty,
}
impl Cell {
    fn is_empty(&self) -> bool {
        match self {
            Cell::Empty => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum GameState {
    Playing,
    Player1win,
    Player2win,
    Draw
}

#[derive(Clone)]
pub struct ArrayBoard {
    cells: [Cell; WIDTH * HEIGHT],
    pub player1: bool,
    pub state: GameState,
    moves: usize,
}

impl ArrayBoard {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            cells: [Cell::Empty; WIDTH * HEIGHT],
            player1: true,
            state: GameState::Playing,
            moves: 0,
        }
    }

    pub fn set_player1(&mut self, x: usize, y: usize) {
        self.cells[x + y*WIDTH] = Cell::Player1;
        self.moves += 1;
    }

    pub fn set_player2(&mut self, x: usize, y: usize) {
        self.cells[x + y*WIDTH] = Cell::Player2;
        self.moves += 1;
    }

    pub fn display_pretty(&self) -> Result<()> {
        let mut stdout = stdout();

        let cols: String = (0..WIDTH).map(|x| x.to_string()).collect();
        stdout.queue(PrintStyledContent(style(cols + "\n")))?;
        for _ in 0..HEIGHT {
            stdout.queue(PrintStyledContent(style("\n")))?;
        }
        stdout.flush()?;

        let (origin_x, origin_y) = crossterm::cursor::position()?;

        for (idx, cell) in self.cells.iter().enumerate() {
            let (pos_x, pos_y) = (
                origin_x + (idx % WIDTH) as u16,
                origin_y - (idx / WIDTH) as u16,
            );

            stdout
                .queue(MoveTo(pos_x, pos_y))?
                .queue(PrintStyledContent(
                    style("0")
                        .attribute(Attribute::Bold)
                        .on(match cell {
                            Cell::Player1 => Color::DarkRed,
                            Cell::Player2 => Color::DarkYellow,
                            Cell::Empty => Color::DarkGrey,
                        })
                        .with(match cell {
                            Cell::Player1 => Color::Red,
                            Cell::Player2 => Color::Yellow,
                            Cell::Empty => Color::White,
                        }),
                ))?;
        }
        stdout
            .queue(MoveTo(origin_x + WIDTH as u16, origin_y))?
            .queue(PrintStyledContent(style("\n")))?;
        stdout.flush()?;
        Ok(())
    }
}