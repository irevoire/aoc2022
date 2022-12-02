use std::ops::Add;

use aoc::{answer, parser};
use parse_display::FromStr;

#[derive(FromStr, PartialEq, Debug, Copy, Clone)]
enum Move {
    #[from_str(regex = "A|X")]
    Rock,
    #[from_str(regex = "B|Y")]
    Paper,
    #[from_str(regex = "C|Z")]
    Scissor,
}

impl Add for Move {
    type Output = usize;

    fn add(self, rhs: Self) -> Self::Output {
        let score = self as usize + 1;

        let outcome = match (self, rhs) {
            // draw
            (Move::Rock, Move::Rock)
            | (Move::Paper, Move::Paper)
            | (Move::Scissor, Move::Scissor) => 3,
            // win
            (Move::Rock, Move::Scissor)
            | (Move::Paper, Move::Rock)
            | (Move::Scissor, Move::Paper) => 6,
            // loss
            (Move::Paper, Move::Scissor)
            | (Move::Scissor, Move::Rock)
            | (Move::Rock, Move::Paper) => 0,
        };

        outcome + score
    }
}

#[derive(FromStr, PartialEq, Debug, Clone, Copy)]
#[display("{prediction} {strategy}")]
struct Match {
    prediction: Move,
    strategy: Move,
}

impl Match {
    pub fn score(self) -> usize {
        self.strategy + self.prediction
    }
}

fn main() {
    let score = parser::lines::<Match>().map(Match::score).sum::<usize>();

    answer!(
        "If everything goes exactly according to strategy the final score should be {}",
        score
    );
}
