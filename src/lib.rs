use std::collections::HashSet;

pub mod algorithms;

const MAX_GUESSES: usize = 32;
const DICTIONARY: &str = include_str!("../dictionary.txt");

pub struct Wordle {
    dictionary: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            dictionary: HashSet::from_iter(DICTIONARY.split_whitespace().step_by(2)),
        }
    }

    pub fn play<G: Guesser>(&self, answer: &'static str, mut guesser: G) -> Option<usize> {
        // play six rounds invoking guesser each round
        let mut history = Vec::new();
        for i in 1..=MAX_GUESSES {
            let guess = guesser.guess(&history);
            if guess == answer {
                return Some(i);
            }
            assert!(self.dictionary.contains(&*guess));
            let correctness = Correctness::compute(answer, &guess);
            history.push(Guess { 
                word: guess, 
                mask: correctness,
            });
        }
        None
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correctness {
    /// Green
    Correct,
    /// Yellow
    Misplaced,
    /// Grey
    Wrong,
}

impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);
        let mut c = [Correctness::Wrong; 5];
        let mut used = [false; 5];
        // Evaluate correctness
        for (i, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                // Correct letter in the right spot
                c[i] = Correctness::Correct;
                used[i] = true;
            } else {
                // Check if there is an unused matching char
                for (j, (a_, g_)) in answer.chars().zip(guess.chars()).enumerate() {
                    if g == a_ && a_ != g_ && !used[j] {
                        c[i] = Correctness::Misplaced;
                        used[j] = true;
                        break;
                    }
                }
            }
        }
        c
    }
}

pub struct Guess {
    pub word: String,
    pub mask: [Correctness; 5],
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

#[cfg(test)]
macro_rules! guesser {
    (|$history:ident| $impl:block) => {{
        struct G;
        impl $crate::Guesser for G {
            fn guess(&mut self, $history: &[Guess]) -> String {
                $impl
            }
        }
        G
    }};
}

#[cfg(test)]
mod tests {
    mod game {
        use crate::{Guess, Wordle};

        #[test]
        fn genius() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| {
                "right".to_string()
            });
            assert_eq!(w.play("right", guesser), Some(1));
        }

        #[test]
        fn magnificent() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 1 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });
            assert_eq!(w.play("right", guesser), Some(2));
        }

        #[test]
        fn impressive() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 2 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });
            assert_eq!(w.play("right", guesser), Some(3));
        }

        #[test]
        fn splendid() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 3 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });
            assert_eq!(w.play("right", guesser), Some(4));
        }

        #[test]
        fn great() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 4 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });
            assert_eq!(w.play("right", guesser), Some(5));
        }

        #[test]
        fn phew() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 5 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });
            assert_eq!(w.play("right", guesser), Some(6));
        }

        #[test]
        fn oops() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| {
                "wrong".to_string()
            });
            assert_eq!(w.play("right", guesser), None);
        }

    }
    mod compute {
        use crate::Correctness;

        macro_rules! mask {
            (C) => {Correctness::Correct};
            (M) => {Correctness::Misplaced};
            (W) => {Correctness::Wrong};
            ($($c:tt)+) => {[
                $(mask!($c)),+
            ]};
        }
        
        #[test]
        fn all_green() {
            assert_eq!(
                Correctness::compute("abcde", "abcde"), mask![C C C C C]
            );
        }

        #[test]
        fn all_grey() {
            assert_eq!(
                Correctness::compute("abcde", "fghij"), mask![W W W W W]
            );
        }

        #[test]
        fn all_yellow() {
            assert_eq!(
                Correctness::compute("abcde", "bcdea"), mask![M M M M M]
            );
        }

        #[test]
        fn repeat_green() {
            assert_eq!(
                Correctness::compute("aabbb", "aaccc"), mask![C C W W W] 
            );
        }

        #[test]
        fn repeat_yellow() {
            assert_eq!(
                Correctness::compute("aabbb", "ccaac"), mask![W W M M W]
            );
        }

        #[test]
        fn repeat_some_green() {
            assert_eq!(
                Correctness::compute("aabbb", "caacc"), mask![W C M W W]
            );
        }

        #[test]
        fn limit_one_yellow() {
            assert_eq!(
                Correctness::compute("azzaz", "aaabb"), mask![C M W W W]
            );
        }
    }
}