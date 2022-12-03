pub struct Rucksack {
    items: String,
}

impl Rucksack {
    pub fn new(s: &str) -> Self {
        if s.len() & 1 == 1 {
            panic!("Rucksack must have an even number of items");
        }
        Self{items: s.to_owned()}
    }

    pub fn duplicate_priority(&self) -> usize {
        let l_2 = self.items.len() / 2;
        for (i, c1) in self.items.chars().enumerate() {
            if i >= l_2 {
                break;
            }
            for c2 in self.items.chars().skip(l_2) {
                if c1 == c2 {
                    return Self::priority(c1);
                }
            }
        }
        0
    }

    pub fn priority(c: char) -> usize {
        if ('A'..='Z').contains(&c) {
            (c as usize) - ('A' as usize) + 27
        } else if ('a'..='z').contains(&c) {
            (c as usize) - ('a' as usize) + 1
        } else {
            0
        }
    }

    pub fn common(&self, other: &Self) -> Vec<char> {
        let cs = other.items.chars().collect();
        self.common_chars(&cs)
    }

    pub fn common_chars(&self, other: &Vec<char>) -> Vec<char> {
        let mut matches = Vec::new();
        for c1 in self.items.chars() {
            for c2 in other {
                if c1 == *c2 && !matches.contains(&c1) {
                    matches.push(c1);
                }
            }
        }
        matches
    }
}
