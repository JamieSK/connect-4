extern crate connect_4;

use connect_4::Connect4;

#[test]
fn it_starts_blank() {
    let game = Connect4::new();
    assert_eq!("\n\
 | | | | | | \n\
 | | | | | | \n\
 | | | | | | \n\
 | | | | | | \n\
 | | | | | | \n\
 | | | | | | \n", game.to_string());
}