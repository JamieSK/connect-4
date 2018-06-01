extern crate connect_4;

use connect_4::Connect4;
use connect_4::Player;

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
    game.play(Player::Yellow, 1).unwrap();
    game.play(Player::Red, 2).unwrap();
    game.play(Player::Yellow, 3).unwrap();
    game.play(Player::Red, 4).unwrap();
    game.play(Player::Yellow, 5).unwrap();
    game.play(Player::Red, 6).unwrap();
    game.play(Player::Yellow, 7).unwrap();
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
    game.play(Player::Yellow, 4).unwrap();
    game.play(Player::Red, 4).unwrap();
    game.play(Player::Yellow, 4).unwrap();
    game.play(Player::Red, 4).unwrap();
    game.play(Player::Yellow, 4).unwrap();
    game.play(Player::Red, 4).unwrap();
    assert_eq!(game.play(Player::Yellow, 4), Err("No more space in that column."));
}