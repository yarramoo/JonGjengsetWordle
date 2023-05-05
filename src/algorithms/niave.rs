use crate::{Guesser, Guess};

pub struct Niave;

impl Niave {
    pub fn new() -> Self {
        Niave
    }
}

impl Guesser for Niave {
    fn guess(&mut self, _history: &[Guess]) -> String {
        todo!();
    }
}