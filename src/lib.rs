use std::fmt;

#[derive(Debug, Clone)]
pub struct Grid {
    pub matrix: [[Player; 3]; 3],
    pub number_of_turns: i32,
    pub player_turn: Player,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Player {
    X,
    O,
    Empty,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::X => 'X',
            Self::O => 'O',
            Self::Empty => ' ',
        };

        write!(f, "{}", c)
    }
}

impl Grid {
    pub fn new() -> Grid {
        return Grid::from([
            [Player::Empty, Player::Empty, Player::Empty],
            [Player::Empty, Player::Empty, Player::Empty],
            [Player::Empty, Player::Empty, Player::Empty],
        ]);
    }

    pub fn from(matrix: [[Player; 3]; 3]) -> Grid {
        let number_of_turn = Grid::count_number_of_turns(matrix);

        let player_turn = if number_of_turn & 1 == 0 {
            Player::X
        } else {
            Player::O
        };
        Grid {
            matrix,
            number_of_turns: number_of_turn,
            player_turn,
        }
    }

    fn count_number_of_turns(matrix: [[Player; 3]; 3]) -> i32 {
        let mut number_of_turn = 0;

        for &row in matrix.iter() {
            for &square in row.iter() {
                match square {
                    Player::Empty => (),
                    Player::O => number_of_turn += 1,
                    Player::X => number_of_turn += 1,
                }
            }
        }

        number_of_turn
    }

    pub fn set(&mut self, x: usize, y: usize) -> Result<(), Player> {
        if self.matrix[y][x] != Player::Empty {
            return Err(self.matrix[y][x]);
        }

        self.matrix[y][x] = self.player_turn;
        self.number_of_turns += 1;
        self.player_turn = if self.player_turn == Player::X {
            Player::O
        } else {
            Player::X
        };

        Ok(())
    }

    pub fn is_full(&self) -> bool {
        self.number_of_turns == 9
    }

    pub fn best_play(&self) -> Option<(usize, usize)> {
        let mut best_play = None;
        let mut best_x = None;
        let mut best_y = None;

        let mut update_best_score = |score, x, y| match best_play {
            Some(best_score) => {
                if best_score < score {
                    best_play = Some(score);
                    best_x = Some(x);
                    best_y = Some(y);
                }
            }
            None => {
                best_play = Some(score);
                best_x = Some(x);
                best_y = Some(y);
            }
        };

        for (y, row) in self.matrix.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                let mut g = self.clone();

                match g.set(x, y) {
                    Ok(_) => {
                        let score = minimax(g);
                        update_best_score(score, x, y);
                    }
                    Err(_) => (),
                }
            }
        }

        let res = match best_play {
            Some(_) => Some((best_x.unwrap(), best_y.unwrap())),
            None => None,
        };

        return res;
    }

    pub fn has_winner(&self) -> bool {
        if self.check_diag() {
            return true;
        }
        for i in 0..3 {
            if self.check_col(i) {
                return true;
            }
            if self.check_row(i) {
                return true;
            }
        }

        return false;
    }

    fn check_col(&self, col: usize) -> bool {
        if self.matrix[0][col] == Player::Empty {
            return false;
        }

        self.matrix[0][col] == self.matrix[1][col] && self.matrix[1][col] == self.matrix[2][col]
    }

    fn check_row(&self, row: usize) -> bool {
        if self.matrix[row][0] == Player::Empty {
            return false;
        }

        self.matrix[row][0] == self.matrix[row][1] && self.matrix[row][1] == self.matrix[row][2]
    }

    fn check_diag(&self) -> bool {
        if self.matrix[0][0] == self.matrix[1][1]
            && self.matrix[1][1] == self.matrix[2][2]
            && self.matrix[0][0] != Player::Empty
        {
            return true;
        }

        if self.matrix[2][0] == self.matrix[1][1]
            && self.matrix[1][1] == self.matrix[0][2]
            && self.matrix[0][2] != Player::Empty
        {
            return true;
        }

        return false;
    }
}

pub fn minimax(grid: Grid) -> i32 {
    if grid.has_winner() {
        if grid.player_turn == Player::X {
            return 10 - grid.number_of_turns;
        } else {
            return grid.number_of_turns - 10;
        }
    }

    if grid.is_full() {
        return 0;
    }

    let mut score = None;

    let mut update_score = |v2| {
        match score {
            Some(v) => {
                if grid.player_turn == Player::X && v > v2 {
                    score = Some(v2);
                }
                if grid.player_turn == Player::O && v < v2 {
                    score = Some(v2);
                }
            },
            None => score = Some(v2),
        }
    };

    for (y, &row) in grid.matrix.iter().enumerate() {
        for (x, &square) in row.iter().enumerate() {
            if square == Player::Empty {
                let mut ng = grid.clone();
                let _ = ng.set(x, y);
                let s = minimax(ng);
                update_score(s);
            }
        }
    }

    return score.unwrap_or(0);
}

pub fn display(grid: &Grid) {
    let m = grid.matrix;
    println!("   1   2   3");
    println!("a  {} | {} | {} ", m[0][0], m[0][1], m[0][2]);
    println!("  -----------");
    println!("b  {} | {} | {} ", m[1][0], m[1][1], m[1][2]);
    println!("  -----------");
    println!("c  {} | {} | {} ", m[2][0], m[2][1], m[2][2]);
}

#[cfg(test)]
mod tests {
    use super::*;

    mod win {
        use super::*;

        #[test]
        fn check_diag() {
            let grid = Grid::from([
                [Player::X, Player::Empty, Player::Empty],
                [Player::O, Player::X, Player::Empty],
                [Player::O, Player::Empty, Player::X],
            ]);
    
            assert_eq!(grid.has_winner(), true);
        }
    
        #[test]
        fn check_antidiag() {
            let grid = Grid::from([
                [Player::O, Player::Empty, Player::X],
                [Player::X, Player::X, Player::Empty],
                [Player::X, Player::O, Player::O],
            ]);
    
            assert_eq!(grid.has_winner(), true);
        }
    
        #[test]
        fn check_col() {
            let grid = Grid::from([
                [Player::O, Player::X, Player::Empty],
                [Player::O, Player::X, Player::X],
                [Player::O, Player::Empty, Player::X],
            ]);
    
            assert_eq!(grid.has_winner(), true);
        }
    
        #[test]
        fn check_row() {
            let grid = Grid::from([
                [Player::X, Player::X, Player::X],
                [Player::O, Player::X, Player::Empty],
                [Player::O, Player::Empty, Player::O],
            ]);
    
            assert_eq!(grid.has_winner(), true);
        }
    
        #[test]
        fn no_winner() {
            let grid = Grid::from([
                [Player::X, Player::Empty, Player::Empty],
                [Player::O, Player::O, Player::X],
                [Player::O, Player::X, Player::X],
            ]);
    
            assert_eq!(grid.has_winner(), false);
        }
    
        #[test]
        fn empty_no_winner() {
            let grid = Grid::from([
                [Player::Empty, Player::Empty, Player::Empty],
                [Player::Empty, Player::Empty, Player::Empty],
                [Player::Empty, Player::Empty, Player::Empty],
            ]);
    
            assert_eq!(grid.has_winner(), false);
        }
    
        #[test]
        fn is_full() {
            let grid = Grid::from([
                [Player::X, Player::X, Player::O],
                [Player::O, Player::O, Player::X],
                [Player::O, Player::X, Player::X],
            ]);
    
            assert_eq!(grid.is_full(), true);
        }
    
        #[test]
        fn is_not_full() {
            let grid = Grid::from([
                [Player::Empty, Player::X, Player::O],
                [Player::O, Player::Empty, Player::X],
                [Player::O, Player::X, Player::X],
            ]);
    
            assert_eq!(grid.is_full(), false);
        }
    }

    mod bot {
        use super::*;

        #[test]
        fn immediate_win() {
            let grid = Grid::from([
                [Player::Empty, Player::O, Player::Empty],
                [Player::X, Player::O, Player::X],
                [Player::Empty, Player::Empty, Player::X],
            ]);
    
            assert_eq!(grid.best_play(), Some((1, 2)));
        }
    
        #[test]
        fn immediate_lose() {
            let grid = Grid::from([
                [Player::O, Player::Empty, Player::X],
                [Player::Empty, Player::Empty, Player::X],
                [Player::Empty, Player::Empty, Player::Empty],
            ]);
    
            assert_eq!(grid.best_play(), Some((2, 2)));
        }
    }

}
