#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MonthDay {
    month: u32,
    day: u32,
}

impl MonthDay {
    pub fn new(month: u32, day: u32) -> Self {
        Self { month, day }
    }

    pub fn from_str(s: &str) -> Self {
        let normalized = s.replace('-', "");

        assert!(
            normalized.len() == 4,
            "Date must be in MMDD or MM-DD format"
        );

        let month: u32 = normalized[..2].parse().unwrap();
        let day: u32 = normalized[2..].parse().unwrap();

        assert!((1..=12).contains(&month), "Invalid month: {}", month);
        assert!((1..=31).contains(&day), "Invalid day: {}", day);

        Self { month, day }
    }    

    pub fn month(&self) -> u32 {
        self.month
    }

    pub fn day(&self) -> u32 {
        self.day
    }
}
