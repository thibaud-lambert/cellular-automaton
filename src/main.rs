mod core;
use core::grid::Grid;
use core::rule::Rule;

use std::env;
use std::fmt;
use std::time::Duration;

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
    let args_list: Vec<_> = env::args().collect();
    if args_list.len() < 3 {
        println!("Not enough arguments : {} grid_size step_number", args_list[0]);
        std::process::exit(1);
    }

    let grid_size = args_list[1].parse::<usize>().unwrap();
    let mut g = Grid::new(grid_size,grid_size,State::DEAD,LifeRule);

    for i in 0..g.n {
        for j in 0..g.m {
            if rand::random::<bool>() {
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
    let dur = Duration::new(0, 4*10u32.pow(8));

    std::thread::sleep(dur);

    let step_number = args_list[2].parse::<usize>().unwrap();
    for i in 0..step_number {
        println!("step:{}",i);
        g = g.step();
        print!("{:?}",g);
        std::thread::sleep(dur);
    }
}
