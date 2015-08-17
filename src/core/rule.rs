pub trait Rule<S> {
    fn process(&self, c : S, neighbor : Vec<S>) -> S;
}
