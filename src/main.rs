use std::io;
use tictactoe::*;

fn main() {
    let grid = Grid::new();
    main_loop(grid);
}

fn main_loop(mut grid: Grid) {
    while !grid.is_full() && !grid.has_winner() {
        let mut x;
        let mut y;
        if grid.player_turn == Player::X {
            display(&grid);
            loop {
                (x, y) = player_turn();
                match grid.set(x as usize, y as usize) {
                    Ok(_) => break,
                    Err(p) => println!("Theses coordinates already have an {p}. Please enter again : "),
                }
            }
        } else {
            match grid.best_play() {
                Some(coo) => { let _ = grid.set(coo.0, coo.1); },
                None => { break; },
            }
        }
    }
    end_game(grid);
}

fn end_game(grid: Grid) {
    let mut winner = Player::X;
    if winner == grid.player_turn {
        winner = Player::O;
    }
    display(&grid);
    if !grid.is_full() {
        println!("Player {} won!", winner);
    } else {
        println!("Draw :(");
    }
}

fn player_turn() -> (i32, i32) {
    println!("Please enter some coordinates : ");
    loop {
        match read_player_trial() {
            Ok(res) => {
                return res;
            },
            Err(e) => println!("{e} : "),
        }
    }
}

fn read_player_trial() -> Result<(i32, i32), &'static str> {
    let mut trial = String::new();

    io::stdin()
        .read_line(&mut trial)
        .expect("Failed to read line");

    let trimmed = trial.trim();
    let mut x = None;
    let mut y = None;

    if trimmed.chars().count() != 2 {
        return Err("Please enter valid coordinates");
    }

    for (i, _) in trimmed.char_indices() {
        let char = &trimmed[i..=i];


        match char.parse::<i32>() {
            Ok(n) => match x {
                None => x = {
                    if n < 4 && n > 0 {
                        Some(n - 1)
                    } else {
                        return Err("Please enter valid coordinates");
                    }
                },
                Some(_) => {
                    return Err("Please enter valid coordinates");
                }
            },
            Err(_) => match y {
                None => {
                    if char.eq("a") {
                        y = Some(0);
                    } else if char.eq("b") {
                        y = Some(1);
                    } else if char.eq("c") {
                        y = Some(2);
                    } else {
                        return Err("Please enter valid coordinates");
                    }
                }
                Some(_) => {
                    return Err("Please enter valid coordinates");
                }
            },
        }
    }

    Ok((x.unwrap(), y.unwrap()))
}
