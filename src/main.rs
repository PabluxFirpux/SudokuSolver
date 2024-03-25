mod cell;
mod sudoku;

fn main() {
    let mut sudoku = sudoku::make_sudoku(9);
    sudoku.print_board();
}
