use std::fmt;

pub struct Connect4 {
    board: Vec<Vec<Option<Player>>>,
    last_player: Option<Player>,
    winning_cells: Vec<Point>,
    pub winner: Option<Player>,
}

#[derive(PartialEq, Debug)]
pub struct Point {
    x: usize,
    y: usize,
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
            winning_cells: vec![],
            winner: None,
        }
    }

    pub fn play(&mut self, player: Player, column: usize) -> Result<&str, &str> {
        if self.last_player == Some(player) {
            return Err("You can't have two goes.");
        } else if column == 0 || column > 7 {
            return Err("That's not in the board.");
        }

        for row in 0..self.board[column - 1].len() {
            match self.board[column - 1][row] {
                Some(_) => continue,
                None => {
                    self.board[column - 1][row] = Some(player);
                    if self.check_win(column - 1, row) {
                        self.winner = Some(player);
                    }
                    self.last_player = Some(player);
                    return Ok("Played a turn.");
                }
            }
        }

        Err("No more space in that column.")
    }

    fn check_win(&mut self, x: usize, y: usize) -> bool {
        let board = &self.board;
        let mut won = false;

        for i in 0..=3 {
            if x + i >= 3 && x + i <= 6
                && board[x + i][y] == board[x + i - 1][y]
                && board[x + i][y] == board[x + i - 2][y]
                && board[x + i][y] == board[x + i - 3][y] {
                self.winning_cells.append(
                    &mut vec![
                        Point {x: x + i, y},
                        Point {x: x + i - 1, y},
                        Point {x: x + i - 2, y},
                        Point {x: x + i - 3, y},
                    ]
                );
                won = true;
            }
            if y + i >= 3 && y + i <= 5
                && board[x][y + i] == board[x][y + i - 1]
                && board[x][y + i] == board[x][y + i - 2]
                && board[x][y + i] == board[x][y + i - 3] {
                self.winning_cells.append(
                    &mut vec![
                        Point {x, y: y + i},
                        Point {x, y: y + i - 1},
                        Point {x, y: y + i - 2},
                        Point {x, y: y + i - 3},
                    ]
                );
                won = true;
            }
            if y + i >= 3 && y + i <= 5 && x + i >= 3 && x + i <= 6
                && board[x + i][y + i] == board[x + i - 1][y + i - 1]
                && board[x + i][y + i] == board[x + i - 2][y + i - 2]
                && board[x + i][y + i] == board[x + i - 3][y + i - 3] {
                self.winning_cells.append(
                    &mut vec![
                        Point {x: x + i, y: y + i},
                        Point {x: x + i - 1, y: y + i - 1},
                        Point {x: x + i - 2, y: y + i - 2},
                        Point {x: x + i - 3, y: y + i - 3},
                    ]
                );
                won = true;
            }
        }

        return won;
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
{}|{}|{}|{}|{}|{}|{}\n\
1|2|3|4|5|6|7\n",
self.cell(1, 6), self.cell(2, 6), self.cell(3, 6), self.cell(4, 6), self.cell(5, 6), self.cell(6, 6), self.cell(7, 6),
self.cell(1, 5), self.cell(2, 5), self.cell(3, 5), self.cell(4, 5), self.cell(5, 5), self.cell(6, 5), self.cell(7, 5),
self.cell(1, 4), self.cell(2, 4), self.cell(3, 4), self.cell(4, 4), self.cell(5, 4), self.cell(6, 4), self.cell(7, 4),
self.cell(1, 3), self.cell(2, 3), self.cell(3, 3), self.cell(4, 3), self.cell(5, 3), self.cell(6, 3), self.cell(7, 3),
self.cell(1, 2), self.cell(2, 2), self.cell(3, 2), self.cell(4, 2), self.cell(5, 2), self.cell(6, 2), self.cell(7, 2),
self.cell(1, 1), self.cell(2, 1), self.cell(3, 1), self.cell(4, 1), self.cell(5, 1), self.cell(6, 1), self.cell(7, 1))
    }

    fn cell(&self, x: usize, y: usize) -> String {
        let contents = &self.board[x - 1][y - 1];
        if self.winning_cells.contains(&Point {x: x - 1, y: y - 1}) {
            match contents {
                None => String::from(" "),
                Some(player) => format!("{}", player.fmt_win()),
            }
        } else {
            match contents {
                None => String::from(" "),
                Some(player) => format!("{}", player),
            }
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

impl Player {
    fn fmt_win(&self) -> String {
        match *self {
            Player::Red => String::from("\x1b[31m*\x1b[0m"),
            Player::Yellow => String::from("\x1b[33m*\x1b[0m"),
        }
    }

    pub fn pretty_print(&self) -> String {
        match *self {
            Player::Red => String::from("\x1b[31mRed\x1b[0m"),
            Player::Yellow => String::from("\x1b[33mYellow\x1b[0m"),
        }
    }
}