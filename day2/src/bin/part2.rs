use std::ops::Add;

use aoc::{answer, parser};
use parse_display::FromStr;

#[derive(FromStr, PartialEq, Debug, Copy, Clone)]
enum Move {
    #[from_str(regex = "A")]
    Rock,
    #[from_str(regex = "B")]
    Paper,
    #[from_str(regex = "C")]
    Scissor,
}

#[derive(FromStr, PartialEq, Debug, Copy, Clone)]
enum Strategy {
    #[from_str(regex = "X")]
    Lose,
    #[from_str(regex = "Y")]
    Draw,
    #[from_str(regex = "Z")]
    Win,
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

impl Add<Strategy> for Move {
    type Output = Move;

    fn add(self, rhs: Strategy) -> Self::Output {
        match (self, rhs) {
            (Move::Rock, Strategy::Draw)
            | (Move::Paper, Strategy::Lose)
            | (Move::Scissor, Strategy::Win) => Move::Rock,

            (Move::Paper, Strategy::Draw)
            | (Move::Scissor, Strategy::Lose)
            | (Move::Rock, Strategy::Win) => Move::Paper,

            (Move::Scissor, Strategy::Draw)
            | (Move::Rock, Strategy::Lose)
            | (Move::Paper, Strategy::Win) => Move::Scissor,
        }
    }
}

#[derive(FromStr, PartialEq, Debug, Clone, Copy)]
#[display("{prediction} {strategy}")]
struct Match {
    prediction: Move,
    strategy: Strategy,
}

impl Match {
    pub fn score(self) -> usize {
        self.prediction + self.strategy + self.prediction
    }
}

fn main() {
    let score = parser::lines::<Match>().map(Match::score).sum::<usize>();

    answer!(
        "If everything goes exactly according to strategy the final score should be {}",
        score
    );
}
