mod cell;
mod sudoku;

fn main() {
    let mut sudoku = sudoku::make_sudoku(9);
    sudoku.set_cell(1,1,5);
    sudoku.set_cell(1,3,7);
    sudoku.print_board();
    println!("{:?}",sudoku.get_square(4));
}
