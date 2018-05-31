use std::fmt;

pub struct Connect4 {
    board: String,
}

pub enum Player {
    Red,
    Yellow,
}

impl Connect4 {
    pub fn new() -> Connect4 {
        Connect4 {
            board: String::from("\n\
 | | | | | | \n\
 | | | | | | \n\
 | | | | | | \n\
 | | | | | | \n\
 | | | | | | \n\
 | | | | | | \n"),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}", self.board)
    }

    pub fn play(&mut self, player: Player, _column: usize) {
        self.board = format!("\n\
 | | | | | | \n\
 | | | | | | \n\
 | | | | | | \n\
 | | | | | | \n\
 | | | | | | \n\
 | | |{}| | | \n", player);
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