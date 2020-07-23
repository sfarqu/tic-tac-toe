use std::io;
use colored::*;

struct Player {
    name: String,
    mark: char,
}

enum Turn {
    Next,
    Invalid,
    Quit
}

fn main() {
    println!("{}", "  Tic-Tac-Toe  ".black().on_cyan());
    println!();

    let mut grid:[[char; 3]; 3] = [[' '; 3]; 3];
    let mut input = String::new(); // is it better to declare in loop or reuse mut here?
    let players = [
        Player{name: "Player 1".to_string(), mark: 'X'},
        Player{name: "Player 2".to_string(), mark: 'O'}
    ];
    let mut current = 0;
    let mut turns = 0;

    menu();

    loop {
        draw_grid(grid);
        println!("{} enter a move: ", players[current].name.blue());
        io::stdin().read_line(&mut input);
        // evaluate player input
        let result = match input.to_uppercase().trim() {
            "A1"|"A2"|"A3"| // surely there's a better way to do this?
            "B1"|"B2"|"B3"|
            "C1"|"C2"|"C3" => game(input.to_uppercase().trim(), &mut grid, &players[current]),
            "Q" => Turn::Quit,
            _ => Turn::Invalid
        };
        // determine whether turn is over
        match result {
            Turn::Invalid => println!("{} {}", "Invalid move".red(), input.to_uppercase().trim().purple()),
            Turn::Quit => break,
            Turn::Next => {
                current = if current == 0 { 1 } else { 0 };
                turns = turns + 1;
            }
        }
        // check for winner
        if let Some(winner) = get_winner(&check_game_over(grid), &players) {
            println!("{}", format!("{} {}", winner.name, "wins!").black().on_green());
            draw_grid(grid);
            break
        }
        // if board is full, end game
        if turns >= 9 {
            println!("{}", format!("{}", "  Draw  ".black().on_magenta()));
            break
        }
        input = "".to_string(); // This seems hacky, is there a more idiomatic way to overwrite values with read_line?
    }
}

fn menu() {
    println!("To play: specify your move with a letter for the row, and a number for the column");
    println!("Example: {}", "B2".purple());
    println!();
    println!("Press enter to start a new game");
    let mut key = String::new();
    io::stdin().read_line(&mut key);
    println!();
}

/**
 * Check if move is on an empty space, if so update board
 */
fn game(mv: &str, grid: &mut [[char; 3]; 3], player: &Player) -> Turn {
    let mut indexes = Vec::new();
    for ch in mv.chars() {
        let i = match ch {
            'A'|'1' => 0,
            'B'|'2' => 1,
            'C'|'3' => 2,
            _ => break // shouldn't happen?
        };
        indexes.push(i);
    }
    match grid[indexes[0]][indexes[1]] {
        ' ' => {
            grid[indexes[0]][indexes[1]] = player.mark;
            return Turn::Next
        },
        _ => return Turn::Invalid
    }
}

fn draw_grid(grid: [[char; 3]; 3]) {
    println!("   1   2   3");
    println!("A  {} | {} | {}", grid[0][0], grid[0][1], grid[0][2]);
    println!("  ---+---+---");
    println!("B  {} | {} | {}", grid[1][0], grid[1][1], grid[1][2]);
    println!("  ---+---+---");
    println!("C  {} | {} | {}", grid[2][0], grid[2][1], grid[2][2]);
}

/**
 * Brute force looking for 3-in-a-row
*/
fn check_game_over(grid: [[char; 3]; 3]) -> char {
    let empty = ' ';
    for i in 0..3 {
        // check rows
        if grid[i][0] != empty && (grid[i][0], grid[i][1]) == (grid[i][1], grid[i][2]) {
            return grid[i][0]
        }
        // check columns
        if grid[0][1] != empty && (grid[0][i], grid[1][i]) == (grid[1][i], grid[2][i]) {
            return grid[0][i]
        }
    }
    // check diagonals
    if grid[0][0] != empty && (grid[0][0], grid[1][1]) == (grid[1][1], grid[2][2]) {
        return grid[1][1]
    }
    if grid[2][0] != empty && (grid[2][0], grid[1][1]) == (grid[1][1], grid[0][2]) {
        return grid[1][1]
    }
    empty
}

/**
* Identify winning player based on who was X or O
*/
fn get_winner<'a,'b>(mark: &'b char, players: &'a [Player; 2]) -> Option<&'a Player> {
    for player in players.iter() {
        if player.mark == *mark { return Some(player); }
    }
    None
}