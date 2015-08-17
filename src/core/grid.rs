extern crate std;
use std::fmt;
use core::rule::Rule;

pub struct Grid<S, R : Rule<S>> {
    rule : R,
    pub n : usize,
    pub m : usize,
    pub cells : Vec<S>,
}

impl<S : Clone + Copy, R : Rule<S> + Clone + Copy> Grid<S,R> {
    pub fn new(n : usize, m : usize, s : S, r : R) -> Grid<S,R> {
        Grid {
            rule : r,
            n : n,
            m : m,
            cells : vec![s;n*m],
        }
    }

    pub fn get(&self, i : usize, j :usize) -> S {
        self.cells[j+i*self.m]
    }

    pub fn get_neighbor(&self, i : usize, j :usize) -> Vec<S> {
        let mut neighbor = Vec::with_capacity(8);
        let imin; let jmin; let imax; let jmax;
        if i > 0 {imin = i-1;} else {imin = 0;}
        if i < self.n-1 {imax = i+2;} else {imax = self.n;}
        if j > 0 {jmin = j-1;} else {jmin = 0;}
        if j < self.m-1 {jmax = j+2;} else {jmax = self.m;}
        for k in imin..imax {
            for l in jmin..jmax {
                if i!=k || j!=l {
                    neighbor.push(self.get(k,l));
                }
            }
        }
        return neighbor;
    }

    pub fn step(self) -> Grid<S,R> {
        let mut g = Grid {
            rule : self.rule,
            n : self.n,
            m : self.m,
            cells : Vec::with_capacity(self.n*self.m)
        };

        for i in 0..g.n {
            for j in 0..g.m {
                let c = self.get(i,j);
                let neighbor = self.get_neighbor(i,j);
                g.cells.push(self.rule.process(c,neighbor));
            }
        }
        return g;
    }
}

impl<S : fmt::Debug, R : Rule<S>> fmt::Debug for Grid<S,R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.n {
            for j in 0..self.m {
                write!(f,"{:?}",self.cells[j+i*self.m]);
            }
            write!(f,"\n");
        }
        write!(f,"\n")
    }
}
