extern crate connect_4;

use connect_4::Connect4;
use connect_4::Player;
use connect_4::State;

#[test]
fn it_starts_blank() {
    let game = Connect4::new();
    assert_eq!("\n | | | | | | \n | | | | | | \n | | | | | | \n | | | | | | \n | | | | | | \n | | | | | | \n",
               game.to_string());
}

#[test]
fn can_play_first_go_as_red() {
    let mut game = Connect4::new();
    game.play(Player::Red, 4).unwrap();
    assert_eq!("\n | | | | | | \n | | | | | | \n | | | | | | \n | | | | | | \n | | | | | | \n | | |\x1b[31mO\x1b[0m| | | \n",
               game.to_string());
}

#[test]
fn can_play_first_go_as_yellow() {
    let mut game = Connect4::new();
    game.play(Player::Yellow, 4).unwrap();
    assert_eq!("\n | | | | | | \n | | | | | | \n | | | | | | \n | | | | | | \n | | | | | | \n | | |\x1b[33mO\x1b[0m| | | \n",
               game.to_string());
}

#[test]
fn can_play_in_any_column() {
    let mut game = Connect4::new();
    play_in_columns(&mut game, vec!(1, 2, 3, 4, 5, 6, 7));
    assert_eq!("\n | | | | | | \n | | | | | | \n | | | | | | \n | | | | | | \n | | | | | | \n\x1b[33mO\x1b[0m|\x1b[31mO\x1b[0m|\x1b[33mO\x1b[0m|\x1b[31mO\x1b[0m|\x1b[33mO\x1b[0m|\x1b[31mO\x1b[0m|\x1b[33mO\x1b[0m\n",
               game.to_string());
}

#[test]
fn can_play_on_top_of_another_counter() {
    let mut game = Connect4::new();
    game.play(Player::Yellow, 4).unwrap();
    game.play(Player::Red, 4).unwrap();
    assert_eq!("\n | | | | | | \n | | | | | | \n | | | | | | \n | | | | | | \n | | |\x1b[31mO\x1b[0m| | | \n | | |\x1b[33mO\x1b[0m| | | \n",
               game.to_string());
}

#[test]
fn cannot_play_as_the_same_player_twice_in_a_row() {
    let mut game = Connect4::new();
    game.play(Player::Yellow, 4).unwrap();
    assert_eq!(game.play(Player::Yellow, 4), Err("You can't have two goes."));
}

#[test]
fn cannot_play_out_the_top() {
    let mut game = Connect4::new();
    play_in_columns(&mut game, vec!(4, 4, 4, 4, 4, 4));
    assert_eq!(game.play(Player::Yellow, 4), Err("No more space in that column."));
}

#[test]
fn game_starts_in_play() {
    let game = Connect4::new();
    assert_eq!(game.state(), State::InPlay);
}

#[test]
fn full_board_is_stalemate() {
    let mut game = Connect4::new();
    play_in_columns(&mut game, vec!(
                         1, 2, 3, 4, 5, 6, 7,
                         2, 3, 4, 5, 6, 7, 1,
                         1, 2, 3, 4, 5, 6, 7,
                         1, 2, 3, 4, 5, 6, 7,
                         2, 3, 4, 5, 6, 7, 1,
                         1, 2, 3, 4, 5, 6, 7));
    assert_eq!(game.state(), State::Stalemate,
               "\nExpected Stalemate; game board was: {}and game winner was: {:?}\n",
               game.to_string(), game.winner);
}

#[test]
fn can_win_horizontally() {
    let mut game = Connect4::new();
    play_in_columns(&mut game, vec!(1, 1, 2, 2, 3, 3, 4));
    assert_eq!(game.state(), State::Won,
               "\nExpected Won; game board was: {}and game winner was: {:?}\n",
               game.to_string(), game.winner);
}

#[test]
fn can_win_vertically() {
    let mut game = Connect4::new();
    play_in_columns(&mut game, vec!(1, 2, 1, 2, 1, 2, 1));
    assert_eq!(game.state(), State::Won,
               "\nExpected Won; game board was: {}and game winner was: {:?}\n",
               game.to_string(), game.winner);
}

#[test]
fn can_win_diagonally() {
    let mut game = Connect4::new();
    play_in_columns(&mut game, vec!(1, 2, 2, 3, 4, 3, 3, 4, 4, 5, 4));
    assert_eq!(game.state(), State::Won,
               "\nExpected Won; game board was: {}and game winner was: {:?}\n",
               game.to_string(), game.winner);
}

fn play_in_columns(game: &mut Connect4, columns: Vec<usize>) {
    let mut player = Player::Yellow;
    for column in columns {
        game.play(player, column).unwrap();
        player = match player {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        };
    }
}