use std::io;
use colored::*;

struct Player {
    name: String,
    mark: char,
}

fn main() {
    let mut grid:[[char; 3]; 3] = [[' '; 3]; 3];
    let mut input = String::new();
    let player1: Player = Player{name: "Player 1".to_string(), mark: 'X'};
    let current_player = &player1;
    loop {
        draw_grid(grid);
        println!("{}: enter a move: ", current_player.name.blue());
        io::stdin().read_line(&mut input);
        let result = match input.to_uppercase().trim() {
            "A1"|"A2"|"A3"| // surely there's a better way to do this?
            "B1"|"B2"|"B3"|
            "C1"|"C2"|"C3" => game(input.to_uppercase().trim(), &mut grid, current_player),
            "Q" => false,
            n @ _ => {
                println!("{} {}", "Invalid move".red(), n.purple());
                true
            }
        };
        if !result { break }
        input = "".to_string(); // This seems hacky, is there a more idiomatic way to overwrite values with read_line?
    }
}

// Check if move is valid, then update grid
fn game(mv: &str, grid: &mut [[char; 3]; 3], player: &Player) -> bool {
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
            grid[indexes[0]][indexes[1]] = player.mark
        },
        _ => println!("{} {}", "Invalid move".red(), mv.purple())
    }
    return true;
}

fn draw_grid(grid: [[char; 3]; 3]) {
    println!("   1   2   3");
    println!("A  {} | {} | {}", grid[0][0], grid[0][1], grid[0][2]);
    println!("  ---+---+---");
    println!("B  {} | {} | {}", grid[1][0], grid[1][1], grid[1][2]);
    println!("  ---+---+---");
    println!("C  {} | {} | {}", grid[2][0], grid[2][1], grid[2][2]);
}