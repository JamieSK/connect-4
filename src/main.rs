extern crate connect_4;

use connect_4::*;
use std::io;

fn main() {
    let mut game = Connect4::new();
    let mut player = Player::Red;

    while game.state() != State::Won {
        print!("{}", game.to_string());
        match take_input(player) {
            column @ 1...7 => {
                match game.play(player, column) {
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    },
                    _ => {},
                }
                player = match player {
                    Player::Red => Player::Yellow,
                    Player::Yellow => Player::Red,
                }
            },
            _ => println!("Can't play there."),
        }
    }

    println!("\nYou win, {:?}.{}", game.winner.unwrap(), game.to_string());
}

fn take_input(player: Player) -> usize {
    println!("Where would you like to play, {}?", player.pretty_print());
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failure to read line.");
    let input_number: usize = match input.trim().parse() {
        Ok(x) => x,
        Err(_) => take_input(player),
    };
    input_number
}
