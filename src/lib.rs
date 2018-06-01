use std::fmt;

pub struct Connect4 {
    board: Vec<Vec<Option<Player>>>,
    last_player: Option<Player>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    Red,
    Yellow,
}

impl Connect4 {
    pub fn new() -> Connect4 {
        Connect4 {
            board: vec![vec![None; 6]; 7],
            last_player: None,
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
                    return Ok("Played a turn.");
                }
            }
        }

        Err("No more space in that column.")
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