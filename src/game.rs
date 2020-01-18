use std::collections::HashSet;

fn find_empty(board: &[Vec<u8>]) -> Option<(u8, u8)> {
    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if board[row][col] == 0 {
                return Some((row as u8, col as u8,));
            }
        }
    }

    return None;
}

fn get_solution(solved: bool, board: &mut Vec<Vec<u8>>) -> (bool, &Vec<Vec<u8>>) {
    let find = find_empty(&board);
    
    if find.is_none() {
        return (true, board);
    }

    let row = find.unwrap().0 as usize;
    let col = find.unwrap().1 as usize;

    for i in 1..10 {
        if is_valid_move(board, (row, col,), i) {
            board[row][col] = i;

            if get_solution(solved, board).0 == true {
                return (true, board,);
            }

            board[row][col] = 0;
        }
    }

    return (false, board);
}

fn print_board(board: &[Vec<u8>]) {
    if !is_valid_board(board) {
        println!("Board is invalid");
        return;
    }

    for row in 0..board.len() {
        if row % 3 == 0 && row != 0 {
            println!("- - - - - - - - - - - - - -");
        }
        for col in 0..board[0].len() {
            if col % 3 == 0 {
                print!("| {} ", board[row][col]);
            } else if col == 8 {
                println!("{} |", board[row][col]);
            } else {
                print!("{} ", board[row][col]);
            }
        }
    }
}

pub fn print_original_board(board: &[Vec<u8>]) {
    print_board(board);
}

pub fn solve_puzzle(board: &mut Vec<Vec<u8>>, print_solution: bool) -> Option<&Vec<Vec<u8>>> {
    if !is_valid_board(board) {
        println!("Board is invalid");
        return None;
    }

    let solution = get_solution(false, board);
    let is_solved = solution.0;
    let solved_board = solution.1;

    if !is_solved {
        println!("Puzzle is unsolvable");
        return None;
    } else if is_solved {
        for i in 0..solved_board.len() { 
            if solved_board[i].contains(&(0 as u8)) {
                println!("Puzzle is unsolvable");
                return None
            }
        }

        if print_solution {
            print_board(&solved_board);
        }

        return Some(solved_board);
    } else {
        return None;
    }
}

fn is_valid_move(board: &[Vec<u8>], position: (usize, usize), num: u8) -> bool {
    // checks if the number is used in a row or column
    for i in 0..board.len() {
        if board[position.0][i] == num ||
                board[i][position.1] == num {
            return false;
        }
    }

    // checks if number is used in a box
    let row = position.0 - position.0 % 3;
    let col = position.1 - position.1 % 3;

    for i in 0..3 {
        for j in 0..3 {
            if board[i+row][j+col] == num {
                return false;
            }
        }
    }

    return true;
}

fn is_valid_board(board: &[Vec<u8>]) -> bool {
    let rows = board[0].len();
    let cols = board[1].len();
    
    // make sure the board is 9 x 9
    if rows != 9 || cols != 9 {
        return false;
    }

    for i in 0..rows{
        // make sure all the values in a row other than 0 are unique
        //let filtered: Vec<u8> = board[i].iter().cloned().filter(|&x| x!=0).collect();
        
        if !is_unique_vector(&board[i]) {
            return false;
        }

        // make sure all numbers are between 0 and 9
        for j in 0..cols {
            if board[i][j] > 9 {
                return false;
            }
        }
    }

    // check for the same number, other than 0, in a column
    let mut transposed_board: Vec<Vec<u8>> = vec![Vec::with_capacity(rows); cols];
    for i in 0..rows {
        for j in 0..cols {
            transposed_board[j].push(board[i][j]);
        }
    }

    for row in transposed_board {
        if !is_unique_vector(&row) {
           return false;
        }
    }

    // makes sure all the numbers, other than 0, in a box are unique
    for row in (0..7).step_by(3) {
        for col in (0..7).step_by(3) {
            let mut board_box: Vec<u8> = Vec::with_capacity(rows);
            for i in 0..3 {
                for j in 0..3 {
                    board_box.push(board[row + i][col + j]);
                }
            }
        
            if !is_unique_vector(&board_box) {
                return false;
            }
        }
    }

    return true;
}

fn is_unique_vector(vector: &Vec<u8>) -> bool {
    let filtered: Vec<u8> = vector.iter().cloned().filter(|&x| x!=0).collect();
    let mut unique = HashSet::new();

    filtered.into_iter().all(move |x| unique.insert(x))
}