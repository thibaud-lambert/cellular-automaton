mod core;
use core::space::Grid;

#[derive(Copy, Clone)]
enum State {
    ALIVE,
    DEAD,
}

fn main() {
    let g = Grid::<State>::new(5,5);
    println!("Hello, world!");
}
