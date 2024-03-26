use crate::sudoku::Sudoku;

mod sudoku_solver;

pub fn solve(mut sudoku: Sudoku) {
    populate(&mut sudoku);
    sudoku_solver::solve(sudoku);
}

fn populate(sudoku: &mut Sudoku) {
    sudoku.set_cell(1,2,2);
    sudoku.set_cell(1,3,6);
    sudoku.set_cell(1,5,3);
    sudoku.set_cell(1,9,8);

    sudoku.set_cell(2,1,9);
    sudoku.set_cell(2,4,6);
    sudoku.set_cell(2,7,1);

    sudoku.set_cell(3,5,1);
    sudoku.set_cell(3,6,9);
    sudoku.set_cell(3,8,4);

    sudoku.set_cell(4,3,7);
    sudoku.set_cell(4,4,3);
    sudoku.set_cell(4,6,2);

    sudoku.set_cell(5,3,4);
    sudoku.set_cell(5,5,7);
    sudoku.set_cell(5,7,8);

    sudoku.set_cell(6,4,8);
    sudoku.set_cell(6,6,6);
    sudoku.set_cell(6,7,7);

    sudoku.set_cell(7,2,5);
    sudoku.set_cell(7,4,7);
    sudoku.set_cell(7,5,2);

    sudoku.set_cell(8,3,9);
    sudoku.set_cell(8,6,5);
    sudoku.set_cell(8,9,4);

    sudoku.set_cell(9,1,4);
    sudoku.set_cell(9,5,6);
    sudoku.set_cell(9,7,2);
    sudoku.set_cell(9,8,1);
}