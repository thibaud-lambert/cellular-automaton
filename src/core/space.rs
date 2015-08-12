extern crate std;

pub trait Cell<S : Copy> {
    fn get_state() -> S;
}

pub trait Space<S : Copy, C : Cell<S>> {
    fn get_neighbor(&self, cell : C) -> Vec<C>;
    fn get_iterator(&mut self) -> std::iter::Iterator<Item=C>;
}

pub struct GridCell<S : Copy> {
    i : usize,
    j : usize,
    state : S,
}

pub struct Grid<S : Copy> {
    n : usize,
    m : usize,
    cells : Vec<S>,
}

pub struct GridIterator<'a, S: Copy + 'a> {
    grid : &'a mut Grid<S>,
    i : usize,
    j : usize,
}

impl<'a, S: Copy> std::iter::Iterator for GridIterator<'a, S> {
    type Item = GridCell<S>;

    fn next(&mut self) -> Option<Self::Item> {
        if(self.j >= self.grid.m) {
            return None;
        } else {
            let item = GridCell {
                i : self.i,
                j : self.j,
                state : self.grid.cells[self.i + self.j*self.grid.n]
            };

            if(self.i < self.grid.n-1) {
                self.i+=1;
            } else {
                self.j+=1;
                self.i=0;
            }

            return Some(item);
        }
    }
}

impl<S: Copy> Grid<S> {
    pub fn new(n : usize, m : usize) -> Grid<S> {
        Grid {
            n : n,
            m : m,
            cells : Vec::with_capacity(n*m),
        }
    }
}
/*
impl<S, C : Cell<S>> Space<S, C> for Grid<S> {
    fn get_neighbor(&self, cell : C) -> Vec<C> {


    }

    fn get_iterator(&mut self) -> std::iter::Iterator<Item=C> {
        return GridIterator {
            grid : self,
            i : 0,
            j : 0,
        }
    }
}
*/
