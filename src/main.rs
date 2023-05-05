use za_wardle::Wordle;

const GAMES: &str = include_str!("../answers.txt");

fn main() {
    for answer in GAMES.split_whitespace() {
        let guesser = za_wardle::algorithms::niave::Niave::new();
        let wordle = Wordle::new();
        wordle.play(answer, guesser);
    }
    println!("hello, world!");
}
