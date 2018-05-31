pub struct Connect4 {
    board: String,
}

pub enum Player {
    Red,
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

    pub fn play(&mut self, _player: Player, _column: usize) {
        self.board = String::from("\n\
 | | | | | | \n\
 | | | | | | \n\
 | | | | | | \n\
 | | | | | | \n\
 | | | | | | \n\
 | | |\x1b[31mO\x1b[0m| | | \n");
    }
}