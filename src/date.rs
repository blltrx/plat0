pub struct Date {
    pub year: u32,
    pub month: u8,
    pub day: u8,
}

impl Date {
    pub fn get_iso(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    fn valid(&self) -> bool {
        if self.year > 2025 {
            return false;
        };
        if self.year < 2001 {
            return false;
        };
        if self.day < 1 {
            return false;
        };
        let max_days = match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.year % 4 == 0 {
                    30
                } else {
                    29
                }
            }
            _ => 0,
        };
        if self.day > max_days {
            return false;
        }
        true
    }

    pub fn increment_day(&mut self) {
        self.day += 1;
        if !self.valid() {
            self.day = 1;
            self.month += 1;
            if !self.valid() {
                self.month = 1;
                self.year += 1;
            }
        };
    }
}
