mod core;
use core::grid::Grid;
use core::rule::Rule;

use std::fmt;

extern crate rand;

#[derive(Copy, Clone)]
enum State {
    ALIVE,
    DEAD,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::ALIVE => write!(f,"0"),
            State::DEAD => write!(f," "),
        }
    }
}

#[derive(Copy, Clone)]
struct LifeRule;
impl Rule<State> for LifeRule {
    fn process(&self, c : State, neighbor : Vec<State>) -> State {
        let mut alive = 0;
        let mut n = neighbor.iter();
        loop {
            match n.next() {
                None => break,
                Some(&State::ALIVE) => alive +=1,
                Some(&State::DEAD) => (),
            }
        };

        match c {
            State::ALIVE => {
                match alive {
                    2 | 3 => State::ALIVE,
                    _ => State::DEAD,
                }
            }
            State::DEAD => {
                match alive {
                    3 => State::ALIVE,
                    _ => State::DEAD,
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let grid_size = 20;
    let mut g = Grid::new(grid_size,grid_size,State::DEAD,LifeRule);

    for i in 0..g.n {
        for j in 0..g.m {
            if(rand::random::<bool>()) {
                g.cells[j+i*g.m] = State::ALIVE;
            }
        }
    }

    for i in (grid_size/2 -2)..(grid_size/2+1) {
        for j in (grid_size/2 -2)..(grid_size/2+1) {
            g.cells[j+i*g.m] = State::ALIVE;
        }
    }

    print!("{:?}",g);
    std::thread::sleep_ms(500);

    for i in 0..100 {
        println!("step:{}",i);
        g = g.step();
        print!("{:?}",g);
        std::thread::sleep_ms(200);
    }
}
