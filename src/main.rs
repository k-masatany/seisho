use crate::game::Game;
use std::io;
mod game;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn main() {
    let mut game = Game::new();

    game.print();

    loop {
        // 入力
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let before = (
            parse_input!(inputs[0], usize),
            parse_input!(inputs[1], usize),
        );
        let after = (
            parse_input!(inputs[2], usize),
            parse_input!(inputs[3], usize),
        );

        game.update(before, after, false);
        game.print();
    }
}
