pub struct Connect4 {}

impl Connect4 {
    pub fn new() -> Connect4 {
        Connect4 {}
    }

    pub fn to_string(&self) -> String {
        String::from("\n\
 | | | | | | \n\
 | | | | | | \n\
 | | | | | | \n\
 | | | | | | \n\
 | | | | | | \n\
 | | | | | | \n")
    }
}