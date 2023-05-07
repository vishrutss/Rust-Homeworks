# Chomp - Homework 3
Vishrut Sharma and Bart Massey 2023

This code plays the game of
[Chomp](https://en.wikipedia.org/wiki/Chomp).  It comprises
a library containing a perfect AI player, together with a
command-line program to play against it.

# Collaborations
Collaborated with Shrikrishna Bhat

# Description
This code plays the game of chomp and provides a library containing a perfect AI player, together with a command-line program to play against it. Chomp is a two-player strategy game played on a rectangular grid made up of smaller square cells, which can be thought of as the blocks of a chocolate bar. The players take it in turns to choose one block and "eat it" (remove from the board), together with those that are below it and to its right. The top left block is "poisoned" and the player who eats this loses (https://en.wikipedia.org/wiki/Chomp).

In this program after the user makes a move the AI implements the Negamax algorithm (https://en.wikipedia.org/wiki/Negamax) to determine it's next move. The code implements recursion to calculate it's move. The base case of the recursion is the ```game_over()``` function which checks the board state to determine of the game is over. The AI follows the following pseudocode to make it's move:
    
``` winning-move(posn):
    for each remaining row r
        for each remaining column c in r
            if r = 0 and c = 0
                continue
            p ← copy of posn
            chomp r, c from p
            m ← winning-move(p)
            if no winning move is returned
                return the move r, c
    return no winning move
```

# Challenges
One of the major challenges while writing the code for the game was figuring out the base case for the ```winning-move()``` function. At first I forgot the base case and the program kept getting stuck in an infinite loop.

Another challenge was writing the code for the computer's move in the ```main.rs``` file. At times it got a little difficult to read the code and understand what was going on when the computer made a move.

# Usage
To play the game we need to provide the dimensions of the board via command line arguments in the format ```cargo run <row_size> <column_size>```.

    Example: cargo run 2 3

Above example produces a board of dimensions 2x3 as follows

    ***
    ***

After creating the board enter move. After the user's move the AI will make it's move. Keep repeating until you win or you lose. Good luck!!

# Testing
The following tests were used to test the program

```
/// Tests winning move function 
fn test_winning_move() {
    let mut c = Chomp::new(2, 2);
    assert!(c.winning_move().is_some());
    c.make_move(1, 1);
    assert!(c.winning_move().is_none());
}
```

```
/// Tests Chomp functions
fn chomp_test() {
    let mut b = Chomp::new(4, 5);
    assert!(b.nrows==4 && b.ncols==5);
    assert!(b.board[0][0]);
    assert!(b.board[3][4]);
    b.make_move(3, 3);
    assert!(!b.board[3][3]);
    assert!(b.board[0][3]);
    assert!(b.board[3][0]);
}
```

# Sources

1) Got the syntax to use command line arguments from: https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
2) Referred the following link to get some help in parsing user moves: https://github.com/pdx-cs-rust/chomp
3) Chomp wikipedia link: https://en.wikipedia.org/wiki/Chomp
4) Interactive Chomp game to help understand the rules: https://www.math.ucla.edu/~tom/Games/chomp.html (Credit to Justin Greever who provided the link on Zulip)

# License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.
