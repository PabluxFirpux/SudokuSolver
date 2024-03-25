use crate::cell::Cell;

#[derive(Debug)]
pub struct Sudoku {
    size: u32,
    board: Vec<Vec<Cell>>
}

impl Sudoku {
    pub fn set_cell(&mut self, x: u32, y: u32, value: u32) {
        let mut cell = &mut self.board[x as usize][y as usize];
        cell.set_value(value);
    }

    pub fn print_board(&mut self) {
        for i in 0..self.size {
            for j in 0..self.size {
                print!("{}", self.board[i as usize][j as usize].value)
            }
            println!("")
        }
    }
}

pub fn make_sudoku(size: u32) -> Sudoku {
    let sudoku = Sudoku {
        size,
        board: make_board(size),
    };

    sudoku
}

fn make_board(size: u32) -> Vec<Vec<Cell>> {
    let mut board = Vec::with_capacity(size as usize);
    for i in 1..=size {
        let mut line = Vec::with_capacity(size as usize);
        for j in 1..=size {
            let cell = Cell {
                value: 0,
                x_pos: i,
                y_pos: j
            };
            line.push(cell);
        }
        board.push(line);
    }
    board
}