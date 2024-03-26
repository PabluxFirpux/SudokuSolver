use crate::sudoku::Sudoku;


struct Shadow {
    num: u32,
    board: Vec<Vec<bool>>
}

impl Shadow {
    fn print_board(&mut self){
        for i in 0..9 {
            if i == 3 || i == 6{
                println!(" ---------------------")
            }
            for j in 0..9 {
                print!(" ");
                if j == 3 || j == 6{
                    print!("| ");
                }
                if !self.board[i][j] {
                    print!("x");
                } else {
                    print!("o");
                }
            }
            println!();
        }
    }
}

pub fn solve(mut sudoku: Sudoku) {
    sudoku.print_board();
    println!("===========================");
    let mut shadow = get_shadow(9, &mut sudoku);
    shadow.print_board();
}

fn get_shadow(num: u32, sudoku: &mut Sudoku) -> Shadow {
    if num > 9 || num < 1 {
        println!("Number invalid");
        std::process::exit(1);
    }

    let mut shadow_board = Vec::with_capacity(9);
    for _ in 0..9 {
        let mut line = Vec::with_capacity(9);
        for j in 0..9 {
            line.push(true);
        }
        shadow_board.push(line);
    }

    let mut blocks = Vec::new();
    for i in 0..sudoku.get_size(){
        let mut row = sudoku.get_row(i+1);
        for j in 0..row.len(){
            if row[j as usize].value == num {
                let mut bl = get_blocks(i+1, (j + 1) as u32);
                blocks.append(&mut bl)

            } else if row[j as usize].value > 0 {
                blocks.push((i+1, (j + 1) as u32))
            }
        }
    }

    for (x,y) in blocks {
        shadow_board[(x-1) as usize][(y-1) as usize] = false;
    }

    Shadow {
        num,
        board: shadow_board
    }
}

fn get_blocks(x: u32, y: u32) -> Vec<(u32, u32)> {
    let mut blocks = Vec::new();
    for i in 1..=9 {
        blocks.push((x,i));
        blocks.push((i,y));
    }
    let newx = (((x-1)/3) * 3);
    let newy = (((y-1)/3) * 3);
    for i in newx..newx+3 {
        for j in newy..newy+3 {
            blocks.push((i+1,j+1));

        }
    }
    blocks
}