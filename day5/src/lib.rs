use std::fmt::Display;

use aoc::termion::{color, style};
use parse_display::FromStr;

#[derive(Debug, Clone, Copy, FromStr)]
#[display("move {number} from {from} to {to}")]
pub struct Instruction {
    pub number: usize,
    pub from: usize,
    pub to: usize,
}

pub struct Crates {
    pub crates: Vec<Vec<char>>,
}

impl std::str::FromStr for Crates {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .lines()
            .rev()
            .skip(1)
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();

        let mut crates = Vec::new();

        for line in s.iter() {
            for (i, c) in line.iter().skip(1).step_by(4).enumerate() {
                if crates.len() <= i {
                    crates.push(Vec::new());
                }

                if c.is_alphabetic() {
                    crates[i].push(*c);
                }
            }
        }

        Ok(Crates { crates })
    }
}

impl Display for Crates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let longest_stack = self.crates.iter().map(|s| s.len()).max().unwrap();

        let mut highest_stack = vec![true; self.crates.len()];

        for i in (0..longest_stack).rev() {
            for (si, stack) in self.crates.iter().enumerate() {
                if let Some(c) = stack.get(i) {
                    if highest_stack[si] {
                        highest_stack[si] = false;

                        write!(
                            f,
                            " [{}{}{}{}{}{}] ",
                            style::Bold,
                            style::Blink,
                            color::Fg(color::Yellow),
                            c,
                            style::Reset,
                            color::Fg(color::LightWhite)
                        )?;
                    } else {
                        write!(f, " [{c}] ")?;
                    }
                } else {
                    write!(f, "     ")?;
                }
            }
            writeln!(f)?;
        }

        for i in 1..self.crates.len() + 1 {
            write!(f, "  {i}  ")?;
        }
        writeln!(f)?;

        Ok(())
    }
}
