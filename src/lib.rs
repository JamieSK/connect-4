use std::fmt;

pub struct Connect4 {
    board: Vec<Vec<Option<Player>>>,
    last_player: Option<Player>,
    pub winner: Option<Player>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
    Red,
    Yellow,
}

#[derive(PartialEq, Debug)]
pub enum State {
    InPlay,
    Stalemate,
    Won,
}

impl Connect4 {
    pub fn new() -> Connect4 {
        Connect4 {
            board: vec![vec![None; 6]; 7],
            last_player: None,
            winner: None,
        }
    }

    pub fn play(&mut self, player: Player, column: usize) -> Result<&str, &str> {
        if self.last_player == Some(player) {
            return Err("You can't have two goes.");
        }
        self.last_player = Some(player);

        for row in 0..self.board[column - 1].len() {
            match self.board[column - 1][row] {
                Some(_) => continue,
                None => {
                    self.board[column - 1][row] = Some(player);
                    if self.is_winning_move(column - 1, row) {
                        self.winner = Some(player);
                    }
                    return Ok("Played a turn.");
                }
            }
        }

        Err("No more space in that column.")
    }

    fn is_winning_move(&self, x: usize, y: usize) -> bool {
        let board = &self.board;
        for i in 0..3 {
            if x + i >= 3 && x + i <= 6
                && board[x + i][y] == board[x + i - 1][y]
                && board[x + i][y] == board[x + i - 2][y]
                && board[x + i][y] == board[x + i - 3][y] {
                return true;
            }
        }
        false
    }

    pub fn state(&self) -> State {
        if self.winner != None {
            return State::Won;
        }
        for column in &self.board {
            if column.contains(&None) {
                return State::InPlay;
            }
        }
        return State::Stalemate;
    }

    pub fn to_string(&self) -> String {
        format!("\n\
{}|{}|{}|{}|{}|{}|{}\n\
{}|{}|{}|{}|{}|{}|{}\n\
{}|{}|{}|{}|{}|{}|{}\n\
{}|{}|{}|{}|{}|{}|{}\n\
{}|{}|{}|{}|{}|{}|{}\n\
{}|{}|{}|{}|{}|{}|{}\n",
self.cell(1, 6), self.cell(2, 6), self.cell(3, 6), self.cell(4, 6), self.cell(5, 6), self.cell(6, 6), self.cell(7, 6),
self.cell(1, 5), self.cell(2, 5), self.cell(3, 5), self.cell(4, 5), self.cell(5, 5), self.cell(6, 5), self.cell(7, 5),
self.cell(1, 4), self.cell(2, 4), self.cell(3, 4), self.cell(4, 4), self.cell(5, 4), self.cell(6, 4), self.cell(7, 4),
self.cell(1, 3), self.cell(2, 3), self.cell(3, 3), self.cell(4, 3), self.cell(5, 3), self.cell(6, 3), self.cell(7, 3),
self.cell(1, 2), self.cell(2, 2), self.cell(3, 2), self.cell(4, 2), self.cell(5, 2), self.cell(6, 2), self.cell(7, 2),
self.cell(1, 1), self.cell(2, 1), self.cell(3, 1), self.cell(4, 1), self.cell(5, 1), self.cell(6, 1), self.cell(7, 1))
    }

    fn cell(&self, x: usize, y: usize) -> String {
        let contents = &self.board[x - 1][y - 1];
        match contents {
            None => String::from(" "),
            Some(player) => format!("{}", player),
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Player::Red => "\x1b[31mO\x1b[0m",
            Player::Yellow => "\x1b[33mO\x1b[0m",
        })
    }
}