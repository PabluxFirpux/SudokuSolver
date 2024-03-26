use crate::cell::Cell;

#[derive(Debug)]
pub struct Sudoku {
    size: u32,
    board: Vec<Vec<Cell>>
}

impl Sudoku {
    pub fn set_cell(&mut self, mut x: u32, mut y: u32, value: u32) {
        x = x - 1;
        y = y - 1;
        let mut cell = &mut self.board[x as usize][y as usize];
        cell.set_value(value);
    }

    pub fn print_board(&mut self) {
        for i in 0..self.size {
            if i == 3 || i == 6{
                println!(" ---------------------")
            }
            for j in 0..self.size {
                print!(" ");
                if j == 3 || j == 6{
                    print!("| ");
                }
                print!("{}", self.board[i as usize][j as usize].value)
            }
            println!();
        }
    }

    pub fn get_row(&mut self, mut n: u32) -> &Vec<Cell> {
        n = n - 1;
        &self.board[n as usize]
    }

    //Maybe problems later
    pub fn get_column(&mut self, mut n: u32) -> Vec<&Cell> {
        n = n - 1;
        let mut vec = Vec::new();
        for i in 0..self.size {
            vec.push(&self.board[i as usize][n as usize])
        }
        vec
    }

    pub fn get_square(&mut self, mut n: u32) -> Vec<Vec<&Cell>> {
        let mut square = Vec::with_capacity(3);
        n = n-1;
        let mut x = (n/3)*3;
        let mut y = (n % 3)*3;
        for i in x..x+3 {
            let mut line = Vec::with_capacity(3);
            for j in y..y+3 {
                line.push(&self.board[i as usize][j as usize])
            }
            square.push(line);
        }
        square
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