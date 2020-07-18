use std::io;

fn main() {
    let mut grid:[[char; 3]; 3] = [[' '; 3]; 3];
    let mut input = String::new();
    loop {
        draw_grid(grid);
        println!("Enter a move: ");
        io::stdin().read_line(&mut input);
        let result = match input.to_uppercase().trim() {
            "A1"|"A2"|"A3"|
            "B1"|"B2"|"B3"|
            "C1"|"C2"|"C3" => game(input.to_uppercase().trim(), &mut grid),
            n @ _ => {
                println!("Invalid move {}! {}", n, n.len());
                false
            }
        };
        if !result { break }
        input = "".to_string();
    }
}

fn game(mv: &str, grid: &mut [[char; 3]; 3]) -> bool {
    let mut indexes = Vec::new();
    for ch in mv.chars() {
        let i = match ch {
            'A'|'1' => 0,
            'B'|'2' => 1,
            'C'|'3' => 2,
            _ => 3
        };
        indexes.push(i);
    }
    match grid[indexes[0]][indexes[1]] {
        ' ' => {
            grid[indexes[0]][indexes[1]] = 'X'
        },
        _ => return false
    }
    println!("You placed an X on {}", mv);
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