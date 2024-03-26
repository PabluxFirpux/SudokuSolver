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
    println!("Unsolved sudoku:");
    sudoku.print_board();
    println!("===========================");
    println!("Solved sudoku:");
    let mut changes = 0;
    let mut last_changes = 0;
    let mut shadows = Vec::new();
    for i in 1..=9{
        shadows.push(get_shadow(i, &mut sudoku));
    }
    changes = apply_shadows(&mut sudoku, shadows);
    while (changes > last_changes) {
        let mut shadows = Vec::new();
        for i in 1..=9{
            shadows.push(get_shadow(i, &mut sudoku));
        }
        last_changes = changes;
        changes = apply_shadows(&mut sudoku, shadows)+last_changes;
    }
    for _ in 0..200 {
        let mut shadows = Vec::new();
        for i in 1..=9{
            shadows.push(get_shadow(i, &mut sudoku));
        }
        last_changes = changes;
        changes = apply_shadows(&mut sudoku, shadows)+last_changes;
    }
    sudoku.print_board();
}

fn apply_shadows(sudoku: &mut Sudoku, shadows: Vec<Shadow>) -> u32 {
    let mut x = 0;
    for shadow in 0..shadows.len(){
        x+=apply_shadow(sudoku, &shadows[shadow]);
    }
    x
}

fn apply_shadow(sudoku: &mut Sudoku, shadow: &Shadow) -> u32 {
    let mut x = 0;
    for i in 1..=9 {
        let mut to_change = Vec::new();
        if is_only(&shadow.board[i-1]){
            x+=1;
            for j in 0..9{
                to_change.push((i-1,j))
            }
        }


        let mut vec = Vec::new();
        for j in 0..9 {
            vec.push(shadow.board[j][i-1])
        }
        if is_only(&vec){
            x+=1;
            for j in 0..9 {
                to_change.push((j,i-1))
            }
        }


        let square = sudoku.get_square(i as u32);
        let mut vec2 = Vec::new();
        for row in &square{
            for cell in row{
                vec2.push(shadow.board[(cell.x_pos-1) as usize][(cell.y_pos-1) as usize]);
            }
        }
        if is_only(&vec2){
            x+=1;
            for row in square{
                for cell in row{
                    to_change.push(((cell.x_pos - 1) as usize, (cell.y_pos - 1) as usize));
                }
            }
        }
        apply_changes(sudoku, shadow, to_change);
    }
    x
}

fn apply_changes(sudoku: &mut Sudoku, shadow: &Shadow, coords: Vec<(usize,usize)>) {
    for (x,y) in coords {
        if shadow.board[x][y] {
            sudoku.set_cell((x + 1) as u32, (y + 1) as u32, shadow.num);
        }
    }
}

fn is_only(bools: &Vec<bool>) -> bool {
    let mut x = 0;
    for b in bools {
        if *b {
            x+=1;
        }
    }
    x == 1
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