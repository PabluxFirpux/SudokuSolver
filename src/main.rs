mod cell;
mod sudoku;
mod solver;

fn main() {
    let mut sudoku = sudoku::make_sudoku(9);
    solver::solve(sudoku);
}
