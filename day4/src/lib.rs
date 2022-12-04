use parse_display::FromStr;

#[derive(FromStr, PartialEq, Debug, Copy, Clone)]
#[display("{first},{second}")]
pub struct Pair {
    pub first: Range,
    pub second: Range,
}

impl Pair {
    pub fn contained(&self) -> bool {
        (self.first.starts <= self.second.starts && self.first.ends >= self.second.ends)
            || (self.second.starts <= self.first.starts && self.second.ends >= self.first.ends)
    }

    pub fn overlap(&self) -> bool {
        self.first.iter().any(|i| self.second.contains(i))
            || self.second.iter().any(|i| self.first.contains(i))
    }
}

#[derive(FromStr, PartialEq, Debug, Copy, Clone)]
#[display("{starts}-{ends}")]
pub struct Range {
    pub starts: usize,
    pub ends: usize,
}

impl Range {
    pub fn contains(&self, value: usize) -> bool {
        (self.starts..=self.ends).contains(&value)
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> {
        self.starts..=self.ends
    }
}
