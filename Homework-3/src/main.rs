//! Command-line Chomp player  
//! Vishrut Sharma and Bart Massey 2023
//!
//! This player repeatedly
//! * Displays the board
//! * Prompts the human for a move until a legal move is obtained
//! * Makes the human move on the board
//! * Displays the board
//! * Gets a winning computer move from the AI
//!   * If the AI has no winning move, chooses a computer move
//!     by going to the last available row and eating the last
//!     available square in that row
//! * Makes the computer move on the board
//! * Displays the computer move
//! This continues until the game is over,
//! at which point either "you lose" or "you win"
//! is printed depending on the outcome.

use chomp_ai::*;
use prompted::input;

/// Display the current board. This should produce output in this format:
///
///    #####
///    #####
///    ####.
///    #....
///
fn show_posn(posn: &Chomp) {
    for r in 0..posn.nrows {
        for c in 0..posn.ncols {
            let state = if posn.board[r][c] { "#" } else { "." };
            print!("{}", state);
        }
        println!();
    }
}

/// Get a move from the human player. The human should
/// supply the move as a row and column (starting from 0)
/// separated by a space, like this.
///
///    2 3
///
/// If the human makes a "bad" move (badly formatted or
/// illegal), this function returns `None`. Otherwise it
/// returns `Some` row and column coordinates of the human
/// move.
fn user_move(posn: &Chomp) -> Option<(usize, usize)> {
    let mv = input!("Enter move: ");
    let row_col: Vec<_> = mv.split_whitespace().collect();
    if row_col.len() != 2 {
        println!("Wrong format!!");
        return None;
    }
    let row: usize = match row_col[0].parse() {
        Err(e) => {
            println!("{}", e);
            return None;
        }
        Ok(v) => v,
    };
    let col: usize = match row_col[1].parse() {
        Err(e) => {
            println!("{}", e);
            return None;
        }
        Ok(v) => v,
    };
    if row > posn.nrows || col > posn.ncols {
        println!(
            "Invalid move. Max rows: {} and Max cols: {}",
            posn.nrows, posn.ncols
        );
        return None;
    }
    if !posn.board[row][col] {
        println!("Invalid move!!");
        return None;
    }
    Some((row, col))
}

/// Play a game, as described above.
///
/// The program should take two command-line arguments
/// representing the board size: a number of rows and a
/// number of columns for the board. The program should fail
/// (somehow) if the requested board size is too large or
/// negative or not numbers etc.
///
/// Thus, a typical run of the program on a 3×4 board might look like
/// ```text
/// cargo run 3 4
/// ```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        error()
    }
    let val1 = &args[1];
    let val2 = &args[2];

    let r = parsenum(val1);
    let c = parsenum(val2);

    let mut board = Chomp::new(r.try_into().unwrap(), c.try_into().unwrap());

    loop {
        //Human's turn
        show_posn(&board);
        let mv = user_move(&board);
        if mv == Some((0, 0)) {
            println!("You lose");
            break;
        }
        let Some((row, col))=mv else {continue;};
        board.make_move(row, col);
    }
}

/// Print a usage error message and exit.
fn error() -> ! {
    eprintln!("chomp usage: <num of rows> <num of columns>");
    std::process::exit(1);
}

/// Parse the given string as a `u64`.
fn parsenum(s: &str) -> u64 {
    s.parse().unwrap_or_else(|_| error())
}
