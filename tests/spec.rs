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
    game.play(Player::Red, 4);
    assert_eq!("\n | | | | | | \n | | | | | | \n | | | | | | \n | | | | | | \n | | | | | | \n | | |\x1b[31mO\x1b[0m| | | \n",
               game.to_string());
}

#[test]
fn can_play_first_go_as_yellow() {
    let mut game = Connect4::new();
    game.play(Player::Yellow, 4);
    assert_eq!("\n | | | | | | \n | | | | | | \n | | | | | | \n | | | | | | \n | | | | | | \n | | |\x1b[33mO\x1b[0m| | | \n",
               game.to_string());
}

#[test]
fn can_play_in_any_column() {
    let mut game = Connect4::new();
    game.play(Player::Yellow, 1);
    game.play(Player::Red, 2);
    game.play(Player::Yellow, 3);
    game.play(Player::Red, 4);
    game.play(Player::Yellow, 5);
    game.play(Player::Red, 6);
    game.play(Player::Yellow, 7);
    assert_eq!("\n | | | | | | \n | | | | | | \n | | | | | | \n | | | | | | \n | | | | | | \n\x1b[33mO\x1b[0m|\x1b[31mO\x1b[0m|\x1b[33mO\x1b[0m|\x1b[31mO\x1b[0m|\x1b[33mO\x1b[0m|\x1b[31mO\x1b[0m|\x1b[33mO\x1b[0m\n",
               game.to_string());
}